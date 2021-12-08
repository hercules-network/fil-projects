extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args{
        out_dir: "src",
        includes: &[],
        input: &["empty.proto","request.proto","rustproto.proto","response.proto","service.proto"],
        //input: &["empty.proto","request.proto","response.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}