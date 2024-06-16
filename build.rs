fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("src/protos");
    config
        .compile_protos(
            &[
                "src/protos/ipc_schema.proto",
                "src/protos/local_schema.proto",
            ],
            &["src/protos"],
        )
        .unwrap();
}
