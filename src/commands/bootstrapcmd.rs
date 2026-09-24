use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;

use crate::commands::internal::sourceutils;

pub fn handle_bootstrap() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    println!("Bootstrapping tinosRSE-Development...");

    println!("Installing Systempackages (build-essential, llvm, mtools, genisoimage)...");
    let apt_status = Command::new("sudo")
        .args(&["apt-get", "update", "-y"])
        .status()
        .and_then(|_| {
            Command::new("sudo")
                .args(&["apt-get", "install", "-y", "build-essential", "llvm", "mtools", "genisoimage"])
                .status()
        });

    match apt_status {
        Ok(status) if status.success() => println!("Systempackages installed sucessfully."),
        _ => {
            eprintln!("Warning: Systempackage installation failed or it was canceled.");
            eprintln!("   Make sure that you have a working internet connection or sudo priviliges");
        }
    }

    println!("Installing Rust-tools and targets...");

    let llvm_tools = Command::new("rustup")
        .args(&["component", "add", "llvm-tools-preview"])
        .status();

    let rust_src = Command::new("rustup")
        .args(&["component", "add", "rust-src"])
        .status();

    let target_add = Command::new("rustup")
        .args(&["target", "add", "x86_64-unknown-none"])
        .status();

    eprintln!("Error: Some Rust-components install failed.");
    eprintln!("   Make sure you have the latest nighly-toolchain");

    sourceutils::fetch_source();

    if let (Ok(s1), Ok(s2), Ok(s3)) = (llvm_tools, rust_src, target_add) {
        if s1.success() && s2.success() && s3.success() {
            println!("All Rust-Components and x86_64-unknown-none installed!");
            println!("\n Environment is Ready! You can compile TinosRSE with the command `cargo run {prog} build`.");
            return;
        }
    }
}