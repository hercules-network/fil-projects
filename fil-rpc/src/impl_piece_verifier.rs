use crate::service_grpc::FilPieceVerifier;
use filecoin_proofs as api_fns;
use filecoin_proofs::types as api_types;
use chrono::prelude::*;
pub struct ImpFilPieceVerifier;

impl FilPieceVerifier for ImpFilPieceVerifier {
    fn verify_piece(&self, o: ::grpc::RequestOptions, p: super::request::FilVerifyPieceRequest) -> ::grpc::SingleResponse<super::response::FilVerifyPieceResponse>{
        println!("{:?} ImpFilPieceVerifier.verify_piece start",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let mut response:super::response::FilVerifyPieceResponse = Default::default();
        let piece_inclusion_proof = p.PieceInclusionProof.as_slice();
        let mut comm_d:[u8;32] = Default::default();
        let mut comm_p:[u8;32] = Default::default();
        let piece_size  = api_types::UnpaddedBytesAmount(p.PieceSize);
        let sector_size = api_types::SectorSize(p.SectorSize);
        println!("{:?} ImpFilPieceVerifier.verify_piece piece_inclusion_proof {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_inclusion_proof.len());
        println!("{:?} ImpFilPieceVerifier.verify_piece comm_d {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_d.len());
        println!("{:?} ImpFilPieceVerifier.verify_piece comm_p {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), comm_p.len());
        println!("{:?} ImpFilPieceVerifier.verify_piece piece_size {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), piece_size.0);
        println!("{:?} ImpFilPieceVerifier.verify_piece sector_size {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), sector_size.0);
        let result = api_fns::verify_piece_inclusion_proof(
            piece_inclusion_proof,
            &comm_d,
            &comm_p,
            piece_size,
            sector_size,
        );
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
        println!("{:?} ImpFilPieceVerifier.verify_piece result IsValid {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.IsValid);
        println!("{:?} ImpFilPieceVerifier.verify_piece result Status {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.Status);
        println!("{:?} ImpFilPieceVerifier.verify_piece result ErrorMsg {}",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), response.ErrorMsg);
        println!("{:?} ImpFilPieceVerifier.verify_piece end",chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        grpc::SingleResponse::completed(response)
    }
}
