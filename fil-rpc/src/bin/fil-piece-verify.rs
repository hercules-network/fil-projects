extern crate grpc;
extern crate failure;
extern crate fil_rpc;
use fil_rpc::*;
use fil_rpc::service_grpc::*;
use std::thread;
use std::net::ToSocketAddrs;
use fil_rpc::request::file_descriptor_proto;
use std::env;

fn main(){
    println!("fil-piece-verify start");
    let args: Vec<String> = env::args().collect();
    assert!(args.len()>1);
    let mut port:u16 = 0;
    match args[1].parse::<u16>() {
        Ok(num) => port = num,
        Err(..) => println!("this was not an integer: {}",args[1]),
    }
    let mut server_builder = grpc::ServerBuilder::new_plain();
    server_builder.http.set_port(port);
    server_builder.add_service(fil_rpc::service_grpc::FilPieceVerifierServer::new_service_def(fil_rpc::impl_piece_verifier::ImpFilPieceVerifier));
    server_builder.http.set_cpu_pool_threads(10);
    let server = server_builder.build().expect("build");
    println!("server stared on addr {}",server.local_addr());
    loop{
        thread::park();
    }
    println!("fil-piece-verify stop");
}