use filecoin_proofs::types::UnpaddedBytesAmount;
use serde::{Deserialize, Serialize};
use storage_proofs::sector::SectorId;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct StagedSectorMetadata {
    pub sector_id: SectorId,
    pub sector_access: String,
    pub pieces: Vec<PieceMetadata>,
    pub seal_status: SealStatus,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct SealedSectorMetadata {
    pub sector_id: SectorId,
    pub sector_access: String,
    pub pieces: Vec<PieceMetadata>,
    pub comm_r_star: [u8; 32],
    pub comm_r: [u8; 32],
    pub comm_d: [u8; 32],
    pub proof: Vec<u8>,
    /// checksum on the whole sector
    pub blake2b_checksum: Vec<u8>,
    /// number of bytes in the sealed sector-file as returned by `std::fs::metadata`
    pub len: u64,
}
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SealParams {
    pub meta: StagedSectorMetadata,
    pub sector_size: u64,
    pub porep_proof_partitions: u8,
    pub prover_id: [u8; 31],
    pub piece_lengths: Vec<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SealCallBackParams{
    pub meta:SealedSectorMetadata,
    pub data:Vec<u8>,
}

impl Default for SealCallBackParams {
    fn default() -> SealCallBackParams {
        SealCallBackParams{
            meta: SealedSectorMetadata{
                sector_id: Default::default(),
                sector_access: Default::default(),
                pieces: Default::default(),
                comm_r_star: Default::default(),
                comm_r: Default::default(),
                comm_d: Default::default(),
                proof: Default::default(),
                blake2b_checksum: Default::default(),
                len:Default::default(),
            },
            data: Default::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PieceMetadata {
    pub piece_key: String,
    pub num_bytes: UnpaddedBytesAmount,
    pub comm_p: Option<[u8; 32]>,
    pub piece_inclusion_proof: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum SealStatus {
    Failed(String),
    Pending,
    Sealed(Box<SealedSectorMetadata>),
    Sealing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SealedSectorHealth {
    Ok,
    ErrorInvalidChecksum,
    ErrorInvalidLength,
    ErrorMissing,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GetSealedSectorResult {
    WithHealth(SealedSectorHealth, SealedSectorMetadata),
    WithoutHealth(SealedSectorMetadata),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SecondsSinceEpoch(pub u64);

impl Default for StagedSectorMetadata {
    fn default() -> StagedSectorMetadata {
        StagedSectorMetadata {
            sector_id: Default::default(),
            sector_access: Default::default(),
            pieces: Default::default(),
            seal_status: SealStatus::Pending,
        }
    }
}
