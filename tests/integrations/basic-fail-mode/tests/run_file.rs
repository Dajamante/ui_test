use std::path::Path;
use ui_test::color_eyre::{eyre::ensure, Result};
use ui_test::*;

#[test]
fn run_file() -> Result<()> {
    let mut config = Config::default();
    // Don't require `extern crate` for imports.
    config.args.push("--edition=2021".into());
    config
        .args
        .push("--out-dir=../../../target/run_file".into());
    let result = ui_test::run_file(
        config,
        &Path::new(file!())
            .parent()
            .unwrap()
            .join("run_file/run_file.rs"),
    )?;
    ensure!(result.success(), "");
    Ok(())
}

#[test]
fn run_file_no_tests() -> Result<()> {
    let path = "../../../target";

    let mut config = Config::default();
    // Don't require `extern crate` for imports.
    config.args.push("--edition=2021".into());
    config
        .args
        .push("--out-dir=../../../target/run_file".into());
    // Don't build a binary, we only provide .rmeta dependencies for now
    config.args.push("--emit=metadata".into());
    config.dependencies_crate_manifest_path = Some("Cargo.toml".into());
    config
        .dependency_builder
        .envs
        .push(("CARGO_TARGET_DIR".into(), path.into()));

    let result = ui_test::run_file(
        config,
        &Path::new(file!())
            .parent()
            .unwrap()
            .join("run_file/run_file_with_deps.rs"),
    )?;
    ensure!(result.success(), "");
    Ok(())
}
