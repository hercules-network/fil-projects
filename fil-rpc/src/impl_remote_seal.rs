use crate::service_grpc::FilRemoteSeal;
use filecoin_proofs as api_fns;
use filecoin_proofs::{types as api_types, seal_extend};
use filecoin_proofs::SealOutputExtend;
use storage_proofs::sector::SectorId;
use sector_builder::metadata::PieceMetadata;
use chrono::prelude::*;
use std::panic::resume_unwind;

pub struct ImplFilRemoteSeal;

impl FilRemoteSeal for ImplFilRemoteSeal{
    fn seal(&self, o: ::grpc::RequestOptions, p: super::request::FilSealRequest) -> ::grpc::SingleResponse<super::response::FilSealResponse>{
        println!("{:?} ImplFilRemoteSeal.seal start",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response:super::response::FilSealResponse = Default::default();
        let sector_size = p.Config.clone().unwrap().SectorSize;
        let ppp = p.Config.clone().unwrap().PoRepProofPartitions as u8;
        let cfg = api_types::PoRepConfig(
            api_types::SectorSize(sector_size),
            api_types::PoRepProofPartitions(ppp));
        let in_data = p.Data.clone();
        let mut prover_id:[u8; 31] = Default::default();
        prover_id.copy_from_slice(p.ProverId.as_slice());
        let sector_id = SectorId::from(p.SectorId);
        let mut piece_lengths_src:Vec<api_types::UnpaddedBytesAmount> = Default::default();
        for piece_len in p.PieceLengths.as_slice().iter(){
            let val = api_types::UnpaddedBytesAmount(*piece_len);
            piece_lengths_src.push(val);
        }
        let mut piece_lengths = piece_lengths_src.as_slice();
        let pieces_src:Vec<PieceMetadata> = Default::default();
        println!("{:?} ImplFilRemoteSeal.seal sector_size {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size);
        println!("{:?} ImplFilRemoteSeal.seal ppp {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), ppp);
        println!("{:?} ImplFilRemoteSeal.seal in_data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), in_data.len());
        println!("{:?} ImplFilRemoteSeal.seal prover_id {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), prover_id.len());
        println!("{:?} ImplFilRemoteSeal.seal piece_lengths {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_lengths.len());
        let SealOutputExtend{
            comm_r,
            comm_r_star,
            comm_d,
            proof,
            comm_ps,
            piece_inclusion_proofs,
            data,
        } = match seal_extend(cfg,in_data,&prover_id,sector_id,piece_lengths) {
            Ok(out) => out,
            Err(e) => panic!("{:?} seal_extend exception:{}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),e)
        };
        let pieces:Vec<PieceMetadata> = pieces_src
            .into_iter()
            .zip(comm_ps.iter())
            .zip(piece_inclusion_proofs.into_iter())
            .map(|((piece, &comm_p), piece_inclusion_proof)| PieceMetadata {
                piece_key: piece.piece_key,
                num_bytes: piece.num_bytes,
                comm_p: Some(comm_p),
                piece_inclusion_proof: Some(piece_inclusion_proof.into()),
            })
            .collect();
        for piece in pieces.iter(){
            let mut val:super::response::FilPieceMetadata = Default::default();
            val.PieceKey = piece.piece_key.clone();
            val.NumBytes = piece.num_bytes.0;
            val.PieceInclusionProof = piece.piece_inclusion_proof.clone().unwrap();
            val.CommP = piece.comm_p.unwrap().to_vec();
            response.Pieces.push(val);
        }
        response.Data = data;
        response.CommD = comm_d.to_vec();
        response.CommR = comm_r.to_vec();
        response.CommRs = comm_r_star.to_vec();
        for ps in comm_ps.iter(){
            response.CommPs.push(ps.to_vec());
        }
        response.Proof = proof;
        response.SectorId = p.SectorId;
        response.ProverId = p.ProverId;
        println!("{:?} ImplFilRemoteSeal.seal response Pieces {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Pieces.len());
        println!("{:?} ImplFilRemoteSeal.seal response Data {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Data.len());
        println!("{:?} ImplFilRemoteSeal.seal response CommD {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.CommD.len());
        println!("{:?} ImplFilRemoteSeal.seal response CommR {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.CommR.len());
        println!("{:?} ImplFilRemoteSeal.seal response CommRs {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.CommRs.len());
        println!("{:?} ImplFilRemoteSeal.seal response Proof {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Proof.len());
        println!("{:?} ImplFilRemoteSeal.seal end",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }
}
