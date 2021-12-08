use std::collections::{BTreeMap, HashSet};
use std::sync::{mpsc, Arc};
use std::thread;

use filecoin_proofs::error::ExpectWithBacktrace;
use filecoin_proofs::{generate_post, PrivateReplicaInfo};
use filecoin_proofs::types::*;
use filecoin_proofs::{SealOutputExtend};
use storage_proofs::sector::SectorId;

use crate::builder::WrappedKeyValueStore;
use crate::error::{err_piecenotfound, err_unrecov, Result};
use crate::helpers::{
    add_piece, get_seal_status, get_sealed_sector_health, get_sectors_ready_for_sealing,
    load_snapshot, persist_snapshot, SnapshotKey,seal_extend,add_piece_extend
};
use crate::helpers::checksum::calculate_checksum;
use crate::kv_store::KeyValueStore;
use crate::metadata::{SealStatus, SealedSectorMetadata, StagedSectorMetadata, SealParams, SealCallBackParams};
use crate::sealer::SealerInput;
use crate::state::{SectorBuilderState, StagedState};
use crate::store::SectorStore;
use crate::GetSealedSectorResult::WithHealth;
use crate::{GetSealedSectorResult, PaddedBytesAmount, SecondsSinceEpoch, UnpaddedBytesAmount};

use std::fs::{copy, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufWriter, Cursor, Read, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const FATAL_NOLOAD: &str = "could not load snapshot";
const FATAL_NORECV: &str = "could not receive task";
const FATAL_NOSEND: &str = "could not send";
const FATAL_SNPSHT: &str = "could not snapshot";
const FATAL_SLRSND: &str = "could not send to sealer";
const FATAL_HUNGUP: &str = "could not send to ret channel";
const FATAL_NOSECT: &str = "could not find sector";

pub struct Scheduler {
    pub thread: Option<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub struct PerformHealthCheck(pub bool);

#[derive(Debug)]
pub enum Request {
    AddPiece(
        String,
        u64,
        String,
        SecondsSinceEpoch,
        mpsc::SyncSender<Result<SectorId>>,
    ),
    AddPieceExtend(String, u64, Vec<u8>, mpsc::SyncSender<Result<SectorId>>),
    GetSealedSectors(
        PerformHealthCheck,
        mpsc::SyncSender<Result<Vec<GetSealedSectorResult>>>,
    ),
    GetStagedSectors(mpsc::SyncSender<Result<Vec<StagedSectorMetadata>>>),
    GetReadySectors(bool,u64,[u8;31],mpsc::SyncSender<Result<Vec<SealParams>>>),
    GetSealStatus(SectorId, mpsc::SyncSender<Result<SealStatus>>),
    GeneratePoSt(
        Vec<[u8; 32]>,
        [u8; 32],      // seed
        Vec<SectorId>, // faults
        mpsc::SyncSender<Result<Vec<u8>>>,
    ),
    RetrievePiece(String, mpsc::SyncSender<Result<Vec<u8>>>),
    SealAllStagedSectors(mpsc::SyncSender<Result<()>>),
    SealExtend(
        PoRepConfig,
        Vec<u8>,
        [u8;31],
        SectorId,
        Vec<UnpaddedBytesAmount>,
        mpsc::SyncSender<Result<SealOutputExtend>>
    ),
    HandleSealResult(SectorId, Box<Result<SealedSectorMetadata>>),
    SealCallBack(SectorId,SealCallBackParams,mpsc::SyncSender<Result<()>>),
    Shutdown,
}

impl Scheduler {
    #[allow(clippy::too_many_arguments)]
    pub fn start_with_metadata<T: 'static + KeyValueStore, S: 'static + SectorStore>(
        scheduler_input_rx: mpsc::Receiver<Request>,
        scheduler_input_tx: mpsc::SyncSender<Request>,
        sealer_input_tx: mpsc::Sender<SealerInput>,
        kv_store: Arc<WrappedKeyValueStore<T>>,
        sector_store: Arc<S>,
        last_committed_sector_id: SectorId,
        max_num_staged_sectors: u8,
        prover_id: [u8; 31],
        sector_size: PaddedBytesAmount,
    ) -> Scheduler {
        let thread = thread::spawn(move || {
            // Build the scheduler's initial state. If available, we
            // reconstitute this state from persisted metadata. If not, we
            // create it from scratch.
            let state = {
                let loaded = load_snapshot(&kv_store, &SnapshotKey::new(prover_id, sector_size))
                    .expects(FATAL_NOLOAD)
                    .map(Into::into);

                loaded.unwrap_or_else(|| SectorBuilderState {
                    staged: StagedState {
                        sector_id_nonce: u64::from(last_committed_sector_id),
                        sectors: Default::default(),
                    },
                    sealed: Default::default(),
                })
            };

            let max_user_bytes_per_staged_sector =
                sector_store.sector_config().max_unsealed_bytes_per_sector();

            let mut m = SectorMetadataManager {
                kv_store,
                sector_store,
                state,
                sealer_input_tx,
                scheduler_input_tx: scheduler_input_tx.clone(),
                max_num_staged_sectors,
                max_user_bytes_per_staged_sector,
                prover_id,
                sector_size,
            };

            loop {
                let task = scheduler_input_rx.recv().expects(FATAL_NORECV);

                // Dispatch to the appropriate task-handler.
                match task {
                    Request::AddPiece(key, amt, path, store_until, tx) => {
                        tx.send(m.add_piece(key, amt, path, store_until))
                            .expects(FATAL_NOSEND);
                    }
                    Request::AddPieceExtend(key,amt,data,tx)=>{
                        tx.send(m.add_piece_extend(key, amt, data)).expects(FATAL_NOSEND);
                    }
                    Request::GetSealStatus(sector_id, tx) => {
                        tx.send(m.get_seal_status(sector_id)).expects(FATAL_NOSEND);
                    }
                    Request::RetrievePiece(piece_key, tx) => m.retrieve_piece(piece_key, tx),
                    Request::GetSealedSectors(check_health, tx) => {
                        tx.send(m.get_sealed_sectors(check_health.0))
                            .expects(FATAL_NOSEND);
                    }
                    Request::GetStagedSectors(tx) => {
                        tx.send(m.get_staged_sectors()).expect(FATAL_NOSEND);
                    }
                    Request::GetReadySectors(seal_all_staged_sectors,sector_size,prover_id,tx) => {
                        tx.send(m.get_ready_sectors(seal_all_staged_sectors,sector_size,prover_id)).expect(FATAL_NOSEND);
                    }
                    Request::SealAllStagedSectors(tx) => {
                        tx.send(m.seal_all_staged_sectors()).expects(FATAL_NOSEND);
                    }
                    Request::SealExtend(porep_conf,in_data,prover_id,sector_id,piece_lengths,tx) => {
                        tx.send(m.seal_extend(porep_conf,in_data,&prover_id,sector_id,piece_lengths.as_slice())).expects(FATAL_NOSEND);
                    }
                    Request::HandleSealResult(sector_id, result) => {
                        m.handle_seal_result(sector_id, *result);
                    }
                    Request::SealCallBack(sector_id, param,tx) => {
                        tx.send(m.seal_callback(sector_id,param)).expects(FATAL_NOSEND);
                    }
                    Request::GeneratePoSt(comm_rs, chg_seed, faults, tx) => {
                        m.generate_post(&comm_rs, &chg_seed, faults, tx)
                    }
                    Request::Shutdown => break,
                }
            }
        });

        Scheduler {
            thread: Some(thread),
        }
    }
}

// The SectorBuilderStateManager is the owner of all sector-related metadata.
// It dispatches expensive operations (e.g. unseal and seal) to the sealer
// worker-threads. Other, inexpensive work (or work which needs to be performed
// serially) is handled by the SectorBuilderStateManager itself.
pub struct SectorMetadataManager<T: KeyValueStore, S: SectorStore> {
    kv_store: Arc<WrappedKeyValueStore<T>>,
    sector_store: Arc<S>,
    state: SectorBuilderState,
    sealer_input_tx: mpsc::Sender<SealerInput>,
    scheduler_input_tx: mpsc::SyncSender<Request>,
    max_num_staged_sectors: u8,
    max_user_bytes_per_staged_sector: UnpaddedBytesAmount,
    prover_id: [u8; 31],
    sector_size: PaddedBytesAmount,
}

impl<T: KeyValueStore, S: SectorStore> SectorMetadataManager<T, S> {
    pub fn generate_post(
        &self,
        comm_rs: &[[u8; 32]],
        challenge_seed: &[u8; 32],
        faults: Vec<SectorId>,
        return_channel: mpsc::SyncSender<Result<Vec<u8>>>,
    ) {
        let fault_set: HashSet<SectorId> = faults.into_iter().collect();

        let comm_rs_set: HashSet<&[u8; 32]> = comm_rs.iter().collect();

        let mut replicas: BTreeMap<SectorId, PrivateReplicaInfo> = Default::default();

        for sector in self.state.sealed.sectors.values() {
            if comm_rs_set.contains(&sector.comm_r) {
                let path_str = self
                    .sector_store
                    .manager()
                    .sealed_sector_path(&sector.sector_access)
                    .to_str()
                    .map(str::to_string)
                    .unwrap();

                let info = if fault_set.contains(&sector.sector_id) {
                    PrivateReplicaInfo::new_faulty(path_str, sector.comm_r)
                } else {
                    PrivateReplicaInfo::new(path_str, sector.comm_r)
                };

                replicas.insert(sector.sector_id, info);
            }
        }

        let output = generate_post(
            self.sector_store.proofs_config().post_config(),
            challenge_seed,
            &replicas,
        );

        // TODO: Where should this work be scheduled? New worker type?
        return_channel.send(output).expects(FATAL_HUNGUP);
    }

    // Unseals the sector containing the referenced piece and returns its
    // bytes. Produces an error if this sector builder does not have a sealed
    // sector containing the referenced piece.
    pub fn retrieve_piece(
        &self,
        piece_key: String,
        return_channel: mpsc::SyncSender<Result<Vec<u8>>>,
    ) {
        let opt_sealed_sector = self.state.sealed.sectors.values().find(|sector| {
            sector
                .pieces
                .iter()
                .any(|piece| piece.piece_key == piece_key)
        });

        if let Some(sealed_sector) = opt_sealed_sector {
            let sealed_sector = Box::new(sealed_sector.clone());
            let task = SealerInput::Unseal(piece_key, sealed_sector, return_channel);

            self.sealer_input_tx
                .clone()
                .send(task)
                .expects(FATAL_SLRSND);
        } else {
            return_channel
                .send(Err(err_piecenotfound(piece_key.to_string()).into()))
                .expects(FATAL_HUNGUP);
        }
    }

    // Returns sealing status for the sector with specified id. If no sealed or
    // staged sector exists with the provided id, produce an error.
    pub fn get_seal_status(&self, sector_id: SectorId) -> Result<SealStatus> {
        get_seal_status(&self.state.staged, &self.state.sealed, sector_id)
    }

    // Write the piece to storage, obtaining the sector id with which the
    // piece-bytes are now associated.
    pub fn add_piece(
        &mut self,
        piece_key: String,
        piece_bytes_amount: u64,
        piece_path: String,
        store_until: SecondsSinceEpoch,
    ) -> Result<SectorId> {
        let destination_sector_id = add_piece(
            &self.sector_store,
            &mut self.state.staged,
            piece_key,
            piece_bytes_amount,
            piece_path,
            store_until,
        )?;

        self.check_and_schedule(false)?;
        self.checkpoint()?;

        Ok(destination_sector_id)
    }
    pub fn add_piece_extend(
        &mut self,
        piece_key: String,
        piece_bytes_amount: u64,
        piece_data: Vec<u8>,
    ) -> Result<SectorId>{
        println!("{:?} sector-builder.scheduler.add_piece_extend start",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let destination_sector_id = add_piece_extend(
            &self.sector_store,
            &mut self.state.staged,
            piece_key,
            piece_bytes_amount,
            piece_data,
        )?;

        println!("{:?} sector-builder.scheduler.add_piece_extend end",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        println!("{:?} sector-builder.scheduler.checkpoint start",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        self.checkpoint()?;
        println!("{:?} sector-builder.scheduler.checkpoint end",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        Ok(destination_sector_id)
    }
    // For demo purposes. Schedules sealing of all staged sectors.
    pub fn seal_all_staged_sectors(&mut self) -> Result<()> {
        self.check_and_schedule(true)?;
        self.checkpoint()
    }

    // Produces a vector containing metadata for all sealed sectors that this
    // SectorBuilder knows about. Includes sector health-information on request.
    pub fn get_sealed_sectors(&self, check_health: bool) -> Result<Vec<GetSealedSectorResult>> {
        use rayon::prelude::*;

        let sectors_iter = self.state.sealed.sectors.values().cloned();

        if !check_health {
            return Ok(sectors_iter
                .map(GetSealedSectorResult::WithoutHealth)
                .collect());
        }

        let with_path: Vec<(PathBuf, SealedSectorMetadata)> = sectors_iter
            .map(|meta| {
                let pbuf = self
                    .sector_store
                    .manager()
                    .sealed_sector_path(&meta.sector_access);

                (pbuf, meta)
            })
            .collect();

        // compute sector health in parallel using workers from rayon global
        // thread pool
        with_path
            .into_par_iter()
            .map(|(pbuf, meta)| {
                let health = get_sealed_sector_health(&pbuf, &meta)?;
                Ok(WithHealth(health, meta))
            })
            .collect()
    }

    // Produces a vector containing metadata for all staged sectors that this
    // SectorBuilder knows about.
    pub fn get_staged_sectors(&self) -> Result<Vec<StagedSectorMetadata>> {
        Ok(self.state.staged.sectors.values().cloned().collect())
    }
    pub fn seal_extend(
        &mut self,
        porep_config: PoRepConfig,
        in_data:Vec<u8>,
        prover_id: &[u8; 31],
        sector_id: SectorId,
        piece_lengths: &[UnpaddedBytesAmount],
    ) -> Result<SealOutputExtend> {
        let result = seal_extend(
            porep_config,
            in_data,
            prover_id,
            sector_id,
            piece_lengths,
        )?;
        Ok(result)
    }

    pub fn get_ready_sectors(&mut self,seal_all_staged_sectors:bool,sector_size:u64,prover_id:[u8;31]) -> Result<Vec<SealParams>> {
        println!("{:?} sector-builder.scheduler.get_ready_sectors start",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let staged_state = &mut self.state.staged;
        let to_be_sealed = get_sectors_ready_for_sealing(
            staged_state,
            self.max_user_bytes_per_staged_sector,
            self.max_num_staged_sectors,
            seal_all_staged_sectors,
        );

        println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),to_be_sealed.len());
        let mut result:Vec<SealParams> = Default::default();
        for sector_id in to_be_sealed {
            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
            let mut sector = staged_state
                .sectors
                .get_mut(&sector_id)
                .expects(FATAL_NOSECT);
            sector.seal_status = SealStatus::Sealing;
            sector.sector_access = self.sector_store.manager().staged_sector_path(&sector.sector_access).display().to_string();

            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop sector_access {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector.sector_access);
            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop section 1", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            let porep_proof_partitions = PoRepProofPartitions::from(self.sector_store.proofs_config().porep_config());
            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop porep_proof_partitions: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), porep_proof_partitions.0);
            let sector_size_out = self.sector_store.proofs_config().porep_config().0;
            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop self.sector_size: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), self.sector_size.0);
            let piece_lengths: Vec<u64> = sector.pieces.iter().map(|p| u64::from(p.num_bytes)).collect();
            let mut param = SealParams {
                meta: sector.clone(),
                sector_size: sector_size_out.0,
                porep_proof_partitions: porep_proof_partitions.0,
                prover_id: prover_id,
                piece_lengths: piece_lengths,
            };
            println!("{:?} sector-builder.scheduler.get_ready_sectors to_be_sealed loop section 2", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            result.push(param);
        }
        println!("{:?} sector-builder.scheduler.get_ready_sectors end",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        Ok(result)
    }

    pub fn seal_callback(
        &mut self,
        sector_id:SectorId,
        param: SealCallBackParams,
    ) -> Result<()> {
        println!("{:?} sector-builder.scheduler.seal_callback start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        println!("{:?} sector-builder.scheduler.seal_callback sector_id: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
        let staged_state = &mut self.state.staged;
        let sealed_state = &mut self.state.sealed;
        let staged_meta = staged_state.sectors.get(&sector_id).unwrap().clone();
        println!("{:?} sector-builder.scheduler.seal_callback section1", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        // Insert the newly-sealed sector into the other state map.
        let mut sealed_sector = param.clone();
        // Provision a new sealed sector access through the manager.
        let sealed_sector_access = match self.sector_store
            .manager()
            .new_sealed_sector_access(sector_id)
            .map_err(failure::Error::from) {
            Ok(ssa) => ssa,
            Err(e) => panic!("{:?} seal_callback sealed_sector_access exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), e)
        };
        println!("{:?} sector-builder.scheduler.seal_callback section2", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let data = param.data.as_slice();
        output(&sealed_sector_access, data);
        println!("{:?} sector-builder.scheduler.seal_callback section3", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        // generate checksum
        let blake2b_checksum = calculate_checksum(&sealed_sector_access)?.as_ref().to_vec();

        // get number of bytes in sealed sector-file
        let len = std::fs::metadata(&sealed_sector_access)?.len();
        sealed_sector.meta.sector_access = sealed_sector_access.clone();
        sealed_sector.meta.pieces = staged_meta.pieces;
        sealed_sector.meta.blake2b_checksum = blake2b_checksum;
        sealed_sector.meta.len = len;
        sealed_state.sectors.insert(sector_id, sealed_sector.meta);
        println!("{:?} sector-builder.scheduler.seal_callback section4", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        // Remove the staged sector from the state map.
        let _ = staged_state.sectors.remove(&sector_id);
        println!("{:?} sector-builder.scheduler.seal_callback section5", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        self.checkpoint().expects(FATAL_SNPSHT);
        println!("{:?} sector-builder.scheduler.seal_callback end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        Ok(())
    }
    // Update metadata to reflect the sealing results.
    pub fn handle_seal_result(
        &mut self,
        sector_id: SectorId,
        result: Result<SealedSectorMetadata>,
    ) {
        // scope exists to end the mutable borrow of self so that we can
        // checkpoint
        {
            let staged_state = &mut self.state.staged;
            let sealed_state = &mut self.state.sealed;

            match result {
                Err(err) => {
                    if let Some(staged_sector) = staged_state.sectors.get_mut(&sector_id) {
                        staged_sector.seal_status =
                            SealStatus::Failed(format!("{}", err_unrecov(err)));
                    };
                }
                Ok(sealed_sector) => {
                    sealed_state.sectors.insert(sector_id, sealed_sector);
                }
            };
        }

        self.checkpoint().expects(FATAL_SNPSHT);
    }

    // Check for sectors which should no longer receive new user piece-bytes and
    // schedule them for sealing.
    fn check_and_schedule(&mut self, seal_all_staged_sectors: bool) -> Result<()> {
        let staged_state = &mut self.state.staged;

        let to_be_sealed = get_sectors_ready_for_sealing(
            staged_state,
            self.max_user_bytes_per_staged_sector,
            self.max_num_staged_sectors,
            seal_all_staged_sectors,
        );

        // Mark the to-be-sealed sectors as no longer accepting data and then
        // schedule sealing.
        for sector_id in to_be_sealed {
            let mut sector = staged_state
                .sectors
                .get_mut(&sector_id)
                .expects(FATAL_NOSECT);
            sector.seal_status = SealStatus::Sealing;

            self.sealer_input_tx
                .clone()
                .send(SealerInput::Seal(
                    sector.clone(),
                    self.scheduler_input_tx.clone(),
                ))
                .expects(FATAL_SLRSND);
        }

        Ok(())
    }

    // Create and persist metadata snapshot.
    fn checkpoint(&self) -> Result<()> {
        persist_snapshot(
            &self.kv_store,
            &SnapshotKey::new(self.prover_id, self.sector_size),
            &self.state,
        )?;

        Ok(())
    }
}

#[allow(dead_code)]
fn output(filename: &str, bytes: &[u8]) -> Result<()> {
    let fp = match OpenOptions::new().truncate(true).create(true).write(true).open(Path::new(filename)) {
        Ok(f) => f,
        Err(e) => panic!("{:?} output FileName:{} exception:{}",SystemTime::now(),filename,e)
    };
    let mut writer = BufWriter::with_capacity( 1024*1024*256, fp);
    writer.write_all( bytes )?;
    writer.write_all( &['\n' as u8] )?;
    writer.flush()?;
    Ok(())
}