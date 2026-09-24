use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;

pub fn handle_build() -> PathBuf {
    println!("Compiling TinosRSE...");

    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Cargo execution failed");

    if !status.success() {
        std::process::exit(1);
    }

    let meta = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("Failed to get Cargo metadata");

    let package = meta
    .root_package()
    .expect("Could not find root package");

let binary = package
    .targets
    .iter()
    .find(|target| target.kind.iter().any(|kind| kind == "bin"))
    .expect("Could not find kernel binary");

let binary_name = binary.name.clone();

// `binary` no longer needs to be borrowed after this point.
let kernel_path = meta
    .target_directory
    .into_std_path_buf()
    .join("debug")
    .join(binary_name);

    println!("Kernel: {}", kernel_path.display());

    kernel_path
}