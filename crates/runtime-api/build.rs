fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src/runtime")
        .compile(&["./proto/app.proto"], &["./proto"])?;
    Ok(())
}
