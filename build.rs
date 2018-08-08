#![feature(rust_2018_preview, use_extern_macros)]
#![warn(rust_2018_idioms)]

fn main() {
    println!("Start Compiling Protos");
    let proto_root = "protos";
    let proto_main = "protos/main.proto";
    println!("cargo:rerun-if-changed={}", proto_main);
    protoc_grpcio::compile_grpc_protos(&["main.proto"], &[proto_root], &proto_root)
        .expect("Failed to compile gRPC definitions!");
}