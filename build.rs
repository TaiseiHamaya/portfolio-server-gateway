use tonic_prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/generated/")
        .compile_protos(&["../../../proto/zone/service.proto"], &["../../../proto"])?;

    Ok(())
}
