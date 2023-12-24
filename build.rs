extern crate prost_build;

fn main() {
    let protos = vec!["proto/helloworld/helloworld.proto"];

    prost_build::compile_protos(
        &protos,
        // The directories under which protoc should look for dependencies in the proto file.
        &[
            "proto/helloworld",
            "proto/googleapis",
            "grpc"
        ],
    ).unwrap_or_else(|e| panic!("failed to compile the proto files: {}", e));

    // Recompile protobufs only if any of the proto files changes.
    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto);
    }
}

