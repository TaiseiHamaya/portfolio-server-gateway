use std::fs;

use tonic_prost_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 出力ファイル・ディレクトリの作成
    fs::create_dir_all("src/generated/server/")?;
    fs::create_dir_all("src/generated/client/")?;

    tonic_prost_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir("src/generated/server/")
        .compile_protos(&["process/zone/event_sync.proto"], &["portfolio-proto"])?;

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/generated/client/")
        .compile_protos(
            &[
                "process/zone/command_sync.proto",
                "process/world/command_zone.proto",
                "process/lobby/service_lobby.proto",
                "process/db/user/service.proto",
                "process/db/record/service.proto",
            ],
            &["portfolio-proto"],
        )?;
    Ok(())
}
