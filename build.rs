fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("src/generated");
    config.protoc_arg("--experimental_allow_proto3_optional");
    config
        .compile_protos(
            &["proto/ipc_schema.proto", "proto/local_schema.proto"],
            &["proto"],
        )
        .unwrap();
}
