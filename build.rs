use std::io::Result;

fn main() -> Result<()> {
    protobuf_codegen::Codegen::new()
        .pure()
        .input("src/protos/schema.proto")
        .cargo_out_dir("protos")
        .include("src")
        .run_from_script();
    Ok(())
}
