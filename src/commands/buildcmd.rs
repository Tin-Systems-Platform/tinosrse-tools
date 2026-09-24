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

    let meta = cargo_metadata::MetadataCommand::new().exec().unwrap();
    meta.target_directory.into_std_path_buf()
}