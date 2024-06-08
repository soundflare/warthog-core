use std::io::Result;

fn main() -> Result<()> {
    protobuf_codegen::Codegen::new()
        .pure()
        .input("src/protos/ipc_schema.proto")
        .input("src/protos/local_schema.proto")
        .cargo_out_dir("protos")
        .include("src")
        .run_from_script();
    Ok(())
}
