use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["proto/user.proto"], &["proto"])?;
    Ok(())
}
