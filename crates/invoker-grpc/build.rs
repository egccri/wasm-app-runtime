fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("D:/github/egccri-runtime/crates/invoker-grpc/src/server")
        .compile(
            &["D:/github/egccri-runtime/crates/invoker-grpc/proto/caller.proto"],
            &["D:/github/egccri-runtime/crates/invoker-grpc/proto"],
        )?;
    Ok(())
}
