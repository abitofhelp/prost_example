extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["proto/helloworld/helloworld.proto"],
        &[
            "proto/helloworld",
            "proto/googleapis",
            "grpc"
        ],
    ).unwrap_or_else(|e| panic!("failed to compile the proto files: {}", e));
}

