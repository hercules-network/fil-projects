use crate::service_grpc::FilPoStVerifier;
use chrono::prelude::*;
use filecoin_proofs as api_fns;
use filecoin_proofs::types as api_types;
use filecoin_proofs::{Commitment};
use crate::helpers::*;
use storage_proofs::sector::SectorId;

pub struct ImplFilPoStVerifier;

impl FilPoStVerifier for ImplFilPoStVerifier {
    fn verify_po_st(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPoStRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPoStResponse> {
        println!("{:?} ImplFilPoStVerifier.verify_po_st start", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response: super::response::FilVerifyPoStResponse = Default::default();
        let sector_size = p.Config.as_ref().unwrap().SectorSize;
        let ppp = p.Config.as_ref().unwrap().PoStProofPartitions as u8;
        let cfg = api_types::PoStConfig(
            api_types::SectorSize(sector_size),
        );
        let crs = p.CommRs.as_slice();
        let mut flattened_comm_rs: Vec<Commitment> = Default::default();
        let mut comm_rs: Vec<[u8; 32]> = Default::default();
        for rs in p.CommRs.iter() {
            let mut val: [u8; 32] = Default::default();
            val.copy_from_slice(&rs[..32]);
            comm_rs.push(val);
            flattened_comm_rs.push(val);
        }
        let mut challenge_seed: [u8; 32] = Default::default();
        challenge_seed.copy_from_slice(p.ChallengeSeed.as_slice());
        let proofs = p.Proofs.as_slice();
        let faults = p.Faults;
        let mut sector_ids: Vec<SectorId> = Default::default();
        for sector_id in p.sector_ids {
            sector_ids.push(SectorId::from(sector_id));
        }
        let mut faulty_sector_ids: Vec<SectorId> = Default::default();
        for secotr_id in p.faulty_sector_ids {
            faulty_sector_ids.push(SectorId::from(secotr_id));
        }
        println!("{:?} ImplFilPoStVerifier.verify_po_st sector_size {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
        println!("{:?} ImplFilPoStVerifier.verify_po_st ppp {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ppp);
        println!("{:?} ImplFilPoStVerifier.verify_po_st comm_rs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_rs.len());
        println!("{:?} ImplFilPoStVerifier.verify_po_st challenge_seed {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), challenge_seed.len());
        println!("{:?} ImplFilPoStVerifier.verify_po_st proofs {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), proofs.len());
        println!("{:?} ImplFilPoStVerifier.verify_po_st faults {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), faults.len());
        let mut replicas = match to_public_replica_info_map(sector_ids, flattened_comm_rs, faulty_sector_ids) {
            Ok(bt) => bt,
            Err(e) => panic!("{:?} to_public_replica_info_map exception:{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), e)
        };
        let result = api_fns::verify_post(cfg, &challenge_seed, proofs, &replicas);
        match result {
            Ok(dynamic) => {
                response.Status = 0;
                response.IsValid = dynamic;
            }
            Err(err) => {
                response.Status = 1;
                response.ErrorMsg = format!("{}", err);
            }
        }
        println!("{:?} ImplFilPoStVerifier.verify_po_st result IsValid {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.IsValid);
        println!("{:?} ImplFilPoStVerifier.verify_po_st result Status {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Status);
        println!("{:?} ImplFilPoStVerifier.verify_po_st result ErrorMsg {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.ErrorMsg);
        println!("{:?} ImplFilPoStVerifier.verify_po_st end", chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }
}