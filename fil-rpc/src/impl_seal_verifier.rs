use crate::service_grpc::FilSealVerifier;
use crate::error::Result;
use chrono::prelude::*;
use filecoin_proofs as api_fns;
use filecoin_proofs::types as api_types;
use filecoin_proofs::constants as api_constants;
use failure;
use storage_proofs::sector::SectorId;
pub struct ImplFilSealVerifier;

impl FilSealVerifier for ImplFilSealVerifier {
    fn verify_seal(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifySealRequest) -> ::grpc::SingleResponse<super::response::FilVerifySealResponse> {
        println!("{:?} ImplFilSealVerifier.verify_seal start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilVerifySealResponse = Default::default();
        let result = porep_proof_partitions_try_from_bytes(p.Proof.as_slice()).and_then(|ppp| {
            let sector_size = p.Config.unwrap().SectorSize;
            let cfg = api_types::PoRepConfig(api_types::SectorSize(sector_size), ppp);
            let mut comm_r: [u8; 32] = Default::default();
            comm_r.copy_from_slice(p.CommR.as_slice());
            let mut comm_d: [u8; 32] = Default::default();
            comm_d.copy_from_slice(p.CommD.as_slice());
            let mut comm_r_star: [u8; 32] = Default::default();
            comm_r_star.copy_from_slice(p.CommRs.as_slice());
            let mut prover_id: [u8; 31] = Default::default();
            prover_id.copy_from_slice(p.ProverId.as_slice());
            let mut sector_id = SectorId::from(p.SectorId);
            let proof = p.Proof.as_slice();
            println!("{:?} ImplFilSealVerifier.verify_seal sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
            println!("{:?} ImplFilSealVerifier.verify_seal comm_r {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_r.len());
            println!("{:?} ImplFilSealVerifier.verify_seal comm_d {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_d.len());
            println!("{:?} ImplFilSealVerifier.verify_seal comm_r_star {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_r_star.len());
            println!("{:?} ImplFilSealVerifier.verify_seal prover_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), prover_id.len());
            println!("{:?} ImplFilSealVerifier.verify_seal sector_id {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_id);
            println!("{:?} ImplFilSealVerifier.verify_seal proof {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), proof.len());
            api_fns::verify_seal(cfg, comm_r, comm_d, comm_r_star, &prover_id, sector_id, proof)
        });
        match result {
            Ok(true) => {
                response.Status = 0;
                response.IsValid = true;
            }
            Ok(false) => {
                response.Status = 0;
                response.IsValid = false;
            }
            Err(err) => {
                response.Status = 1;
                response.ErrorMsg = format!("{}", err);
            }
        }
        println!("{:?} ImplFilSealVerifier.verify_seal result IsValid {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.IsValid);
        println!("{:?} ImplFilSealVerifier.verify_seal result Status {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Status);
        println!("{:?} ImplFilSealVerifier.verify_seal result ErrorMsg {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.ErrorMsg);
        println!("{:?} ImplFilSealVerifier.verify_seal end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }
}

fn porep_proof_partitions_try_from_bytes(proof: &[u8]) -> Result<api_types::PoRepProofPartitions> {
    let n = proof.len();
    ensure!(
       n % api_constants::SINGLE_PARTITION_PROOF_LEN == 0,
       "no PoRepProofPartitions mapping for {:x?}",
        proof
   );
    let result =api_types::PoRepProofPartitions(
        (n / api_constants::SINGLE_PARTITION_PROOF_LEN) as u8,
    );
    Ok(result)
    //let result:u8 = (n / api_constants::SINGLE_PARTITION_PROOF_LEN) as u8;
    //if result == 0{
    //     ensure!(
    //    );
    //    Err("no PoRepProofPartitions mapping for {:x?}",proof)
    // }
    //Ok(result);
}