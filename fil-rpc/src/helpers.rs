use filecoin_proofs::types as api_types;
use filecoin_proofs::{constants as api_constants, Commitment, PublicReplicaInfo};
use crate::error::Result;
use failure;
use sector_builder::{SectorBuilderErr, SectorManagerErr};
use std::collections::BTreeMap;
use std::collections::HashSet;
use storage_proofs::sector::SectorId;
/// Return the number of partitions used to create the given proof.
///

pub fn porep_proof_partitions_try_from_bytes(
    proof: &[u8],
) -> Result<api_types::PoRepProofPartitions> {
    let n = proof.len();
    ensure!(
       n % api_constants::SINGLE_PARTITION_PROOF_LEN == 0,
       "no PoRepProofPartitions mapping for {:x?}",
        proof
   );
    Ok(api_types::PoRepProofPartitions(
        (n / api_constants::SINGLE_PARTITION_PROOF_LEN) as u8,
    ))
    //let result:u8 = (n / api_constants::SINGLE_PARTITION_PROOF_LEN) as u8;
    //if result == 0{
    //     ensure!(
    //    );
    //    Err("no PoRepProofPartitions mapping for {:x?}",proof)
    // }
    //Ok(result);
}

pub fn into_safe_challenge_seed(challenge_seed: &[u8; 32]) -> [u8; 32] {
    let mut cs = [0; 32];
    cs.copy_from_slice(challenge_seed);
    cs[31] &= 0b0011_1111;
    cs
}

pub fn to_public_replica_info_map(sector_ids:Vec<SectorId>,flattened_comm_rs:Vec<Commitment>,faulty_sector_ids:Vec<SectorId>) ->Result<BTreeMap<SectorId, PublicReplicaInfo>> {
    let mut m = BTreeMap::new();
    for i in 0..sector_ids.len() {
        m.insert(sector_ids[i],
                 if faulty_sector_ids.contains(&sector_ids[i]) {
                     PublicReplicaInfo::new_faulty(flattened_comm_rs[i])
                 } else {
                     PublicReplicaInfo::new(flattened_comm_rs[i])
                 },
        );
    }
    Ok(m)
}

pub fn err_code_match_status(err: &failure::Error) -> super::response::FilResponseStatus {
    match err.downcast_ref() {
        Some(SectorBuilderErr::OverflowError { .. }) => return super::response::FilResponseStatus::CallerError,
        Some(SectorBuilderErr::IncompleteWriteError { .. }) => return super::response::FilResponseStatus::ReceiverError,
        Some(SectorBuilderErr::Unrecoverable(_, _)) => return super::response::FilResponseStatus::ReceiverError,
        Some(SectorBuilderErr::PieceNotFound(_)) => return super::response::FilResponseStatus::CallerError,
        None => (),
    }
    match err.downcast_ref() {
        Some(SectorManagerErr::UnclassifiedError(_)) => return super::response::FilResponseStatus::UnclassifiedError,
        Some(SectorManagerErr::CallerError(_)) => return super::response::FilResponseStatus::CallerError,
        Some(SectorManagerErr::ReceiverError(_)) => return super::response::FilResponseStatus::ReceiverError,
        None => (),
    }
    super::response::FilResponseStatus::UnclassifiedError
}