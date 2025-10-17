use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_prost_build::configure().compile_protos(
        &["../proto/email_service.proto", "../proto/auth_service.proto"],
        &["../proto"],
    )?;

    Ok(())
}
