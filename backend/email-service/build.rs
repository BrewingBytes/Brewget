use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir(PathBuf::from(env::var("OUT_DIR").unwrap()))
        .compile_protos(&["proto/email_service.proto"], &["proto"])?;

    Ok(())
}
