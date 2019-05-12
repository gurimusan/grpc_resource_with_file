extern crate protoc_grpcio;

#[cfg(feature = "genproto")]
mod inner {
    pub fn gen() {
        let proto_root = "src/protos";
        println!("cargo:rerun-if-changed={}", proto_root);
        protoc_grpcio::compile_grpc_protos(
            &["resource_with_file.proto"],
            &[proto_root],
            &proto_root
        ).expect("Failed to compile gRPC definitions!");
    }
}

#[cfg(not(feature = "genproto"))]
mod inner {
    pub fn gen() {}
}

fn main() {
    inner::gen();
}
