use crate::service_grpc::FilBuilder;
use crate::helpers::*;
use filecoin_proofs as api_fns;
use filecoin_proofs::{types as api_types, UnpaddedBytesAmount, SectorSize, PoRepProofPartitions};
use protobuf::{SingularPtrField, RepeatedField};
use sector_builder::{SealStatus, SectorBuilder, SealCallBackParams, SealedSectorMetadata, PieceMetadata, SecondsSinceEpoch,GetSealedSectorResult,SealedSectorHealth};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;
use std::io;
use std::fs::{copy, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufWriter, Cursor, Read, SeekFrom, BufReader};
use std::path::{Path, PathBuf};
use memmap::MmapMut;
use memmap::MmapOptions;
use crate::response::{FilSealedSectorMetadata, FilPieceMetadata, FilSealedSectorHealth};
use crate::request::{FilSealRequest, FilPoRepConfig};
use storage_proofs::sector::SectorId;
pub struct ImplFilBuilder;

pub static mut builder:Option<Arc<Mutex<SectorBuilder>>> = None;

impl FilBuilder for ImplFilBuilder {
    fn init(&self, o: ::grpc::RequestOptions, p: super::request::FilInitSectorBuilderRequest) -> ::grpc::SingleResponse<super::response::FilInitSectorBuilderResponse> {
        println!("{:?} ImplFilBuilder.init start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilInitSectorBuilderResponse = Default::default();
        let conf = p.SectorClass.clone().unwrap();
        let sector_size = conf.SectorSize;
        let porep_proof_partitions: u8 = conf.PoRepProofPartitions as u8;
        let post_proof_partitions: u8 = conf.PoStProofPartitions as u8;
        let sector_class = filecoin_proofs::SectorClass(api_types::SectorSize(sector_size), api_types::PoRepProofPartitions(porep_proof_partitions));
        let mut prover_id: [u8; 31] = Default::default();
        prover_id.copy_from_slice(p.ProverId.as_slice());
        let max_num_staged_sectors: u8 = p.MaxNumStagedSectors as u8;
        let staged_sectir_dir = p.StagedSectorDir;
        let sealed_sector_dir = p.SealedSectorDir;
        let metadata_dir = p.MetaDataDir;
        let last_used_sector_id = SectorId::from(p.LastCommittedSectorId);
        println!("{:?} ImplFilBuilder.init sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
        println!("{:?} ImplFilBuilder.init porep_proof_partitions {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), porep_proof_partitions);
        println!("{:?} ImplFilBuilder.init post_proof_partitions {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), post_proof_partitions);
        println!("{:?} ImplFilBuilder.init prover_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), prover_id.len());
        println!("{:?} ImplFilBuilder.init max_num_staged_sectors {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), max_num_staged_sectors);
        println!("{:?} ImplFilBuilder.init staged_sectir_dir {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), staged_sectir_dir);
        println!("{:?} ImplFilBuilder.init sealed_sector_dir {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sealed_sector_dir);
        println!("{:?} ImplFilBuilder.init metadata_dir {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), metadata_dir);
        println!("{:?} ImplFilBuilder.init last_used_sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), last_used_sector_id);
        let result = SectorBuilder::init_from_metadata(
            sector_class,
            last_used_sector_id,
            metadata_dir,
            prover_id,
            sealed_sector_dir,
            staged_sectir_dir,
            max_num_staged_sectors,
        );
        match result {
            Ok(value) => {
                response.Status = super::response::FilResponseStatus::NoError;
                unsafe { builder = Some(Arc::new(Mutex::new(value))); }
                println!("{:?} ImplFilBuilder.init finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
            Err(err) => {
                println!("ImplFilBuilder.init error: {}", err);
                response.Status = err_code_match_status(&err);
                response.ErrorMsg = format!("{}", err);
            }
        }
        println!("{:?} ImplFilBuilder.init end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn add_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse> {
        println!("{:?} ImplFilBuilder.add_piece start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilAddPieceResponse = Default::default();
        let piece_key = p.PieceKey;
        let bytes_amount = p.PieceBytesAmount;
        let piece_path = p.PiecePath;
        let store_until = p.SecondsSinceEpoch;
        println!("{:?} ImplFilBuilder.add_piece piece_key {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_key);
        println!("{:?} ImplFilBuilder.add_piece bytes_amount {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), bytes_amount);
        println!("{:?} ImplFilBuilder.add_piece piece_path {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_path);
        println!("{:?} ImplFilBuilder.add_piece store_until {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), store_until);
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.add_piece builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().add_piece(piece_key, bytes_amount, piece_path, SecondsSinceEpoch(store_until));
                match result {
                    Ok(sector_id) => {
                        response.SectorId = u64::from(sector_id);
                        response.Status = super::response::FilResponseStatus::NoError;
                        println!("{:?} ImplFilBuilder.add_piece result sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
                        println!("{:?} ImplFilBuilder.add_piece finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.add_piece builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.add_piece end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn add_piece_extend(&self, o: ::grpc::RequestOptions, p: super::request::FilAddPieceExtendRequest) -> ::grpc::SingleResponse<super::response::FilAddPieceResponse> {
        println!("{:?} ImplFilBuilder.add_piece_extend start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilAddPieceResponse = Default::default();
        let piece_key = p.PieceKey;
        let piece_bytes_amount = p.PieceBytesAmount;
        let data = p.data;
        println!("{:?} ImplFilBuilder.add_piece_extend piece_key {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_key);
        println!("{:?} ImplFilBuilder.add_piece_extend bytes_amount {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_bytes_amount);
        println!("{:?} ImplFilBuilder.add_piece_extend data {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.add_piece_extend builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().add_piece_extend(piece_key, piece_bytes_amount, data);
                match result {
                    Ok(sector_id) => {
                        response.SectorId = u64::from(sector_id);
                        response.Status = super::response::FilResponseStatus::NoError;
                        println!("{:?} ImplFilBuilder.add_piece_extend result sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
                        println!("{:?} ImplFilBuilder.add_piece_extend finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.add_piece_extend builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.add_piece_extend end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn read_piece_from_sealed_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilReadPieceFromSealedSectorRequest) -> ::grpc::SingleResponse<super::response::FilReadPieceFromSealedSectorResponse> {
        println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilReadPieceFromSealedSectorResponse = Default::default();
        let piece_key = p.PieceKey.clone();
        println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector piece_key {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_key);
        let mut response: super::response::FilReadPieceFromSealedSectorResponse = Default::default();
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().read_piece_from_sealed_sector(piece_key);
                match result {
                    Ok(piece_bytes) => {
                        response.Data = piece_bytes;
                        response.Status = super::response::FilResponseStatus::NoError;
                        println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector result piece_bytes {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Data.len());
                        println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.read_piece_from_sealed_sector end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn get_seal_status(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealStatusRequest) -> ::grpc::SingleResponse<super::response::FilGetSealStatusResponse> {
        println!("{:?} ImplFilBuilder.get_seal_status start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGetSealStatusResponse = Default::default();
        let sector_id = SectorId::from(p.SectorId);
        println!("{:?} ImplFilBuilder.get_seal_status sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.get_seal_status builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().get_seal_status(sector_id);
                match result {
                    Ok(seal_status) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        match seal_status {
                            SealStatus::Sealed(meta) => {
                                let meta = meta;
                                let mut ref_meta: super::response::FilSealedSectorMetadata = Default::default();
                                ref_meta.SectorId = u64::from(meta.sector_id);
                                ref_meta.SectorAccess = meta.sector_access;
                                ref_meta.CommRs.copy_from_slice(&meta.comm_r_star);
                                ref_meta.CommR.copy_from_slice(&meta.comm_r);
                                ref_meta.CommD.copy_from_slice(&meta.comm_d);
                                ref_meta.Proof = meta.proof;
                                let sector_access = ref_meta.SectorAccess.clone();
                                //let mut sealed_path = Path::new(&sector_access);
                                //let display = path.display();
                                let mut f_in = match File::open(&sector_access) {
                                    Ok(f) => f,
                                    Err(e) => panic!("{:?} no such file {} exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                                };
                                let mut data = Vec::new();
                                match f_in.read_to_end(&mut data) {
                                    Ok(d) => d,
                                    Err(e) => panic!("{:?} no such file {} exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                                };

                                ref_meta.SealedData = data;
                                for piece in meta.pieces.iter() {
                                    let mut net_piece: super::response::FilPieceMetadata = Default::default();
                                    net_piece.PieceKey = piece.piece_key.clone();
                                    net_piece.NumBytes = u64::from(piece.num_bytes);
                                    ref_meta.Pieces.push(net_piece);
                                }
                                response.SealMetaData = SingularPtrField::some(ref_meta);
                                response.SealStatus = super::response::FilSealStatus::Sealed;
                                response.Status = super::response::FilResponseStatus::NoError;
                                println!("{:?} ImplFilBuilder.get_seal_status finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                println!("{:?} ImplFilBuilder.get_seal_status sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), meta.sector_id);
                                println!("{:?} ImplFilBuilder.get_seal_status SectorAccess {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_access);
                                println!("{:?} ImplFilBuilder.get_seal_status CommRs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), meta.comm_r_star.len());
                                println!("{:?} ImplFilBuilder.get_seal_status CommR {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), meta.comm_r.len());
                                println!("{:?} ImplFilBuilder.get_seal_status CommD {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), meta.comm_d.len());
                                //println!("{:?} ImplFilBuilder.get_seal_status Proof {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), meta.proof.len());
                                //println!("{:?} ImplFilBuilder.get_seal_status data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
                                println!("{:?} ImplFilBuilder.get_seal_status SealStatus Sealed", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                            }
                            SealStatus::Sealing => {
                                response.SealStatus = super::response::FilSealStatus::Sealing;
                                println!("{:?} ImplFilBuilder.get_seal_status SealStatus SealStatus", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                            }
                            SealStatus::Pending => {
                                response.SealStatus = super::response::FilSealStatus::Pending;
                                println!("{:?} ImplFilBuilder.get_seal_status SealStatus Pending", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                            }
                            SealStatus::Failed(err) => {
                                response.SealStatus = super::response::FilSealStatus::Failed;
                                response.ErrorMsg = format!("{}", err);
                                println!("{:?} ImplFilBuilder.get_seal_status SealStatus Failed", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                println!("{:?} ImplFilBuilder.get_seal_status SealStatus Failed exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), err);
                            }
                        }
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.get_seal_status builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.get_seal_status end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn get_sealed_sectors(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSealedSectorsRequest) -> ::grpc::SingleResponse<super::response::FilGetSealedSectorsResponse> {
        println!("{:?} ImplFilBuilder.get_sealed_sectors start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGetSealedSectorsResponse = Default::default();
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.get_sealed_sectors builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().get_sealed_sectors(p.PerformHealthchecks);
                match result {
                    Ok(sealed_sectors) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        for sealed in sealed_sectors.iter() {
                            let (health,meta) = match sealed {
                                GetSealedSectorResult::WithHealth(h, m) => {
                                    match h {
                                        SealedSectorHealth::Ok => (FilSealedSectorHealth::Ok,m),
                                        SealedSectorHealth::ErrorInvalidChecksum => (FilSealedSectorHealth::ErrorInvalidChecksum,m),
                                        SealedSectorHealth::ErrorInvalidLength => (FilSealedSectorHealth::ErrorInvalidLength,m),
                                        SealedSectorHealth::ErrorMissing => (FilSealedSectorHealth::ErrorMissing,m),
                                    }
                                },
                                GetSealedSectorResult::WithoutHealth(m) => (FilSealedSectorHealth::Unknown,m)
                            };
                            let mut ref_meta: super::response::FilSealedSectorMetadata = Default::default();
                            ref_meta.SectorId =  u64::from(meta.sector_id);
                            ref_meta.SectorAccess = meta.sector_access.clone();
                            ref_meta.CommRs.copy_from_slice(&meta.comm_r_star);
                            ref_meta.CommR.copy_from_slice(&meta.comm_r);
                            ref_meta.CommD.copy_from_slice(&meta.comm_d);
                            ref_meta.Proof = meta.proof.clone();
                            ref_meta.Health = health;
                            //let mut sector_access = ref_meta.SectorAccess.clone();
                            //let mut sealed_path = Path::new(&sector_access);
                            //let display = path.display();
                            //let mut f_in = match File::open(&sector_access) {
                            //    Ok(f) => f,
                            //    Err(e) => panic!("{:?} no such file {} exception:{}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                            //};
                            //let mut data = Vec::new();
                            //match f_in.read_to_end(&mut data) {
                            //    Ok(d) => d,
                            //    Err(e) => panic!("{:?} no such file {} exception:{}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                            //};
                            //ref_meta.SealedData = data;
                            for piece in meta.pieces.iter() {
                                let mut net_piece: super::response::FilPieceMetadata = Default::default();
                                net_piece.PieceKey = piece.piece_key.clone();
                                net_piece.NumBytes = u64::from(piece.num_bytes);
                                ref_meta.Pieces.push(net_piece);
                            }
                            println!("{:?} ImplFilBuilder.get_sealed_sectors finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                            println!("{:?} ImplFilBuilder.get_sealed_sectors sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.SectorId);
                            println!("{:?} ImplFilBuilder.get_sealed_sectors SectorAccess {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.SectorAccess);
                            println!("{:?} ImplFilBuilder.get_sealed_sectors CommRs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.CommRs.len());
                            println!("{:?} ImplFilBuilder.get_sealed_sectors CommR {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.CommR.len());
                            println!("{:?} ImplFilBuilder.get_sealed_sectors CommD {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.CommD.len());
                            println!("{:?} ImplFilBuilder.get_sealed_sectors Proof {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.Proof.len());
                            //println!("{:?} ImplFilBuilder.get_sealed_sectors data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
                            response.MetaData.push(ref_meta);
                        }
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.get_sealed_sectors builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.get_sealed_sectors end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn get_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilGetStagedSectorsResponse> {
        println!("{:?} ImplFilBuilder.get_staged_sectors start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGetStagedSectorsResponse = Default::default();
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.get_staged_sectors builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().get_staged_sectors();
                match (result) {
                    Ok(staged_sectors) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        for staged in staged_sectors.iter() {
                            let mut ref_meta: super::response::FilStagedSectorMetadata = Default::default();
                            ref_meta.SectorId = u64::from(staged.sector_id);
                            ref_meta.SectorAccess = staged.sector_access.clone();
                            ref_meta.Pieces = Default::default();
                            //let mut sector_access =  ref_meta.SectorAccess.clone();
                            //let mut sealed_path = Path::new(&sector_access);
                            //let display = path.display();
                            //let mut f_in = match File::open(&sector_access) {
                            //    Ok(f) => f,
                            //     Err(e) => panic!("{:?} ImplFilBuilder.get_staged_sectors no such file {} exception:{}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                            //};
                            //let mut data = Vec::new();
                            //match f_in.read_to_end(&mut data) {
                            //    Ok(d) => d,
                            //    Err(e) => panic!("{:?} ImplFilBuilder.get_staged_sectors no such file {} exception:{}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access, e)
                            //};
                            for piece in staged.pieces.iter() {
                                let mut net_piece: super::response::FilPieceMetadata = Default::default();
                                net_piece.PieceKey = piece.piece_key.clone();
                                net_piece.NumBytes = u64::from(piece.num_bytes);
                                ref_meta.Pieces.push(net_piece);
                            }
                            match staged.seal_status {
                                SealStatus::Failed(ref s) => {
                                    ref_meta.Status = super::response::FilSealStatus::Failed;
                                    println!("{:?} ImplFilBuilder.get_staged_sectors Status Failed", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                }
                                SealStatus::Sealing => {
                                    ref_meta.Status = super::response::FilSealStatus::Sealing;
                                    println!("{:?} ImplFilBuilder.get_staged_sectors Status Sealing", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                }
                                SealStatus::Pending => {
                                    ref_meta.Status = super::response::FilSealStatus::Pending;
                                    println!("{:?} ImplFilBuilder.get_staged_sectors Status Pending", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                }
                                SealStatus::Sealed(_) => {
                                    ref_meta.Status = super::response::FilSealStatus::Sealed;
                                    println!("{:?} ImplFilBuilder.get_staged_sectors Status Sealed", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                                }
                            }
                            println!("{:?} ImplFilBuilder.get_staged_sectors SectorId {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.SectorId);
                            println!("{:?} ImplFilBuilder.get_staged_sectors SectorAccess {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.SectorAccess);
                            //println!("{:?} ImplFilBuilder.get_staged_sectors data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
                            println!("{:?} ImplFilBuilder.get_staged_sectors Pieces {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_meta.Pieces.len());
                            response.MetaData.push(ref_meta);
                        }
                        response.Status = super::response::FilResponseStatus::NoError;
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.get_staged_sectors builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.get_staged_sectors end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn seal_all_staged_sectors(&self, o: ::grpc::RequestOptions, p: super::empty::Empty) -> ::grpc::SingleResponse<super::response::FilSealAllStagedSectorsResponse> {
        println!("{:?} ImplFilBuilder.seal_all_staged_sectors start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilSealAllStagedSectorsResponse = Default::default();
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.seal_all_staged_sectors builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().seal_all_staged_sectors();
                match result {
                    Ok(_) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        println!("{:?} ImplFilBuilder.seal_all_staged_sectors finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.seal_all_staged_sectors builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.seal_all_staged_sectors end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn get_max_user_bytes_per_staged_sector(&self, o: ::grpc::RequestOptions, p: super::request::FilGetMaxUserBytesPerStagedSectorRequest) -> ::grpc::SingleResponse<super::response::FilGetMaxUserBytesPerStagedSectorResponse> {
        println!("{:?} ImplFilBuilder.get_max_user_bytes_per_staged_sector start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGetMaxUserBytesPerStagedSectorResponse = Default::default();
        let sector_size = p.SectorSize.clone();
        println!("{:?} ImplFilBuilder.get_max_user_bytes_per_staged_sector sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
        let out = u64::from(api_types::UnpaddedBytesAmount::from(api_types::SectorSize(
            sector_size,
        )));
        println!("{:?} ImplFilBuilder.get_max_user_bytes_per_staged_sector out {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), out);
        response.SectorSize = out.clone();
        println!("{:?} ImplFilBuilder.get_max_user_bytes_per_staged_sector sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.SectorSize);
        println!("{:?} ImplFilBuilder.get_max_user_bytes_per_staged_sector end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn get_soon_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilGetSoonSealRequest) -> ::grpc::SingleResponse<super::request::FilGetSoonSealResponse> {
        println!("{:?} ImplFilBuilder.get_soon_seal start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::request::FilGetSoonSealResponse = Default::default();
        let seal_all_staged_sectors = p.SealAllStagedSectors;
        println!("{:?} ImplFilBuilder.get_soon_seal seal_all_staged_sectors {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), seal_all_staged_sectors);
        let mut prover_id: [u8; 31] = Default::default();
        prover_id.copy_from_slice(p.ProverId.as_slice());
        println!("{:?} ImplFilBuilder.get_soon_seal prover_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), prover_id.len());
        let sector_size = p.SectorSize;
        println!("{:?} ImplFilBuilder.get_soon_seal sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.get_soon_seal builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().get_ready_sectors(seal_all_staged_sectors, sector_size, &prover_id);
                match result {
                    Ok(out) => {
                        println!("{:?} ImplFilBuilder.get_soon_seal out {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), out.len());
                        for param in out.iter() {
                            let mut cfg: FilPoRepConfig = Default::default();
                            //cfg.SectorSize = param.sector_size;
                            cfg.SectorSize = sector_size;
                            cfg.PoRepProofPartitions = param.porep_proof_partitions as u64;
                            println!("{:?} ImplFilBuilder.get_soon_seal param.sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), cfg.SectorSize);
                            let mut sector_access = param.meta.sector_access.clone();
                            let mut sector_access_tip = param.meta.sector_access.clone();
                            println!("{:?} ImplFilBuilder.get_soon_seal param.sector_access {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_access);
                            //let mut sealed_path = Path::new(&sector_access);
                            //let display = path.display();
                            let mut f_in = match OpenOptions::new().read(true).write(true).open(PathBuf::from(sector_access)) {
                                Ok(f) => f,
                                Err(e) => panic!("{:?} ImplFilBuilder.get_soon_seal no such file {} exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), &sector_access_tip, e)
                            };
                            //f_in.set_len(sector_size);
                            let mut f_data = unsafe { MmapOptions::new().map_mut(&f_in).unwrap() };
                            println!("{:?} ImplFilBuilder.get_soon_seal sector_size: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
                            //let mut leaves_data: Vec<u8> = Vec::with_capacity(sector_bytes as usize);
                            let mut data: Vec<u8> = f_data.to_vec();
                            println!("{:?} ImplFilBuilder.get_soon_seal data: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
                            let mut pieces: RepeatedField<FilPieceMetadata> = Default::default();
                            for piece in param.meta.pieces.iter() {
                                let mut net_piece: super::response::FilPieceMetadata = Default::default();
                                net_piece.PieceKey = piece.piece_key.clone();
                                net_piece.NumBytes = u64::from(piece.num_bytes);
                                pieces.push(net_piece);
                            }
                            //let piece_lengths = param.meta.pieces.iter().map(|p| u64::from(p.num_bytes)).collect();
                            println!("{:?} ImplFilBuilder.get_soon_seal piece_lengths: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), param.piece_lengths.len());
                            let val: FilSealRequest = FilSealRequest {
                                Config: SingularPtrField::some(cfg),
                                Data: data,
                                ProverId: param.prover_id.to_vec(),
                                SectorId: u64::from(param.meta.sector_id),
                                Pieces: pieces,
                                PieceLengths: param.piece_lengths.clone(),
                                unknown_fields: ::protobuf::UnknownFields::new(),
                                cached_size: ::protobuf::CachedSize::default(),
                            };
                            response.Seals.push(val);
                            //println!("{:?} ImplFilBuilder.get_soon_seal SectorSize {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), val.Config.unwrap().SectorSize);
                            //println!("{:?} ImplFilBuilder.get_soon_seal PoRepProofPartitions {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ref_seal.Config.unwrap().PoRepProofPartitions);
                            //println!("{:?} ImplFilBuilder.get_soon_seal sector_access {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_access);
                            //println!("{:?} ImplFilBuilder.get_soon_seal data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
                            //println!("{:?} ImplFilBuilder.get_soon_seal pieces {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), val.Pieces.len());
                            //println!("{:?} ImplFilBuilder.get_soon_seal piece_lengths {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_lengths.len());
                            //println!("{:?} ImplFilBuilder.get_soon_seal ProverId {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), val.ProverId.len());
                        }
                        response.Status = super::response::FilResponseStatus::NoError;
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.get_soon_seal builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.get_soon_seal end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn seal_call_back(&self, o: ::grpc::RequestOptions, p: super::request::FilSealCallBackRequest) -> ::grpc::SingleResponse<super::response::FilSealCallBackResponse> {
        println!("{:?} ImplFilBuilder.seal_call_back start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilSealCallBackResponse = Default::default();
        let mut seal_result = &p.SealResult.unwrap();
        let sector_id = SectorId::from(seal_result.SectorId);
        let mut data = seal_result.clone().Data;
        //let mut sector_access = p.SealedSectorAccess.clone();
        let mut proof = seal_result.clone().Proof;
        let mut comm_r: [u8; 32] = Default::default();
        comm_r.copy_from_slice(seal_result.clone().CommR.as_slice());
        let mut comm_d: [u8; 32] = Default::default();
        comm_d.copy_from_slice(seal_result.clone().CommD.as_slice());
        let mut comm_rs: [u8; 32] = Default::default();
        comm_rs.copy_from_slice(seal_result.clone().CommRs.as_slice());
        let mut ref_meta: SealedSectorMetadata = Default::default();
        ref_meta.comm_d = comm_d;
        ref_meta.comm_r = comm_r;
        ref_meta.comm_r_star = comm_rs;
        ref_meta.proof = proof;
        //ref_meta.sector_access = sector_access;
        ref_meta.sector_id = sector_id;
        //for piece in seal_result.clone().Pieces.iter(){
        //    let mut comm_p:[u8;32] = Default::default();
        //    comm_p.copy_from_slice(piece.CommP.as_slice());
        //    let val:PieceMetadata = PieceMetadata{
        //       num_bytes:UnpaddedBytesAmount(piece.NumBytes),
        //        piece_key:piece.PieceKey.clone(),
        //    };
        //   ref_meta.pieces.push(val);
        //}
        let param = SealCallBackParams {
            meta: ref_meta,
            data: data,
        };
        println!("{:?} ImplFilBuilder.seal_call_back sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
        //println!("{:?} ImplFilBuilder.seal_call_back data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), data.len());
        //println!("{:?} ImplFilBuilder.seal_call_back sector_access {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_access.clone());
        //println!("{:?} ImplFilBuilder.seal_call_back proof {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), proof.len());
        println!("{:?} ImplFilBuilder.seal_call_back comm_r {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_r.clone().len());
        println!("{:?} ImplFilBuilder.seal_call_back comm_d {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_d.len());
        println!("{:?} ImplFilBuilder.seal_call_back comm_rs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_rs.len());
        println!("{:?} ImplFilBuilder.seal_call_back param.pieces {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), param.meta.pieces.len());
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.seal_call_back builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().seal_callback(sector_id, param);
                match result {
                    Ok(_) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        println!("{:?} ImplFilBuilder.seal_call_back finish", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.seal_call_back builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.seal_call_back end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn generate_piece_commitent(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePieceCommitentRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePieceCommitentResponse> {
        println!("{:?} ImplFilBuilder.generate_piece_commitent start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGeneratePieceCommitentResponse = Default::default();
        let unpadded_piece_data = p.UnpaddedPieceData;
        let unpadded_piece_size: UnpaddedBytesAmount = UnpaddedBytesAmount(p.UnpaddedPieceSize);
        println!("{:?} ImplFilBuilder.generate_piece_commitent unpadded_piece_data {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), unpadded_piece_data.len());
        println!("{:?} ImplFilBuilder.generate_piece_commitent unpadded_piece_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), p.UnpaddedPieceSize);
        let result = match api_fns::generate_piece_commitment_extend(unpadded_piece_data, unpadded_piece_size) {
            Ok(piece_commitment) => piece_commitment,
            Err(e) => panic!("{:?} ImplFilBuilder.generate_piece_commitent exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), e),
        };
        response.Data = result.to_vec();
        println!("{:?} ImplFilBuilder.generate_piece_commitent result {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), result.len());
        println!("{:?} ImplFilBuilder.generate_piece_commitent end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }

    fn generate_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilGeneratePoStRequest) -> ::grpc::SingleResponse<super::response::FilGeneratePoStResponse> {
        println!("{:?} ImplFilBuilder.generate_po_st start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilGeneratePoStResponse = Default::default();
        let mut comm_rs_in: Vec<[u8; 32]> = Default::default();
        for comm_r in p.CommRs.iter() {
            let mut val: [u8; 32] = Default::default();
            val.copy_from_slice(comm_r.as_slice());
            comm_rs_in.push(val);
        }
        let comm_rs = comm_rs_in.as_slice();
        let mut challenge_seed: [u8; 32] = Default::default();
        challenge_seed.copy_from_slice(p.ChallengeSeed.as_slice());
        let mut faults: Vec<SectorId> = Default::default();
        for fault in p.Faults.iter() {
            faults.push(SectorId::from(*fault))
        }
        println!("{:?} ImplFilBuilder.generate_po_st comm_rs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_rs.len());
        println!("{:?} ImplFilBuilder.generate_po_st challenge_seed {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), challenge_seed.len());
        unsafe {
            if let Some(ref instance) = builder {
                println!("{:?} ImplFilBuilder.generate_po_st builder have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                let result = instance.lock().unwrap().generate_post(comm_rs, &challenge_seed, faults);
                match result {
                    Ok(proofs) => {
                        response.Status = super::response::FilResponseStatus::NoError;
                        response.Proofs = proofs;
                    }
                    Err(err) => {
                        response.Status = err_code_match_status(&err);
                        response.ErrorMsg = format!("{}", err);
                    }
                }
            } else {
                println!("{:?} ImplFilBuilder.generate_po_st builder not have", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        println!("{:?} ImplFilBuilder.generate_po_st proofs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Proofs.len());
        println!("{:?} ImplFilBuilder.generate_po_st end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }
}