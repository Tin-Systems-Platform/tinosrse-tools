use std::process::Command;
use std::env;

use crate::commands::internal::sourceutils;

fn resolve_nix_binary() -> Option<String> {
    let nix_in_path = Command::new("nix")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if nix_in_path {
        return Some("nix".to_string());
    }

    let default_nix_path = "/nix/var/nix/profiles/default/bin/nix";
    let nix_in_default_path = Command::new(default_nix_path)
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if nix_in_default_path {
        return Some(default_nix_path.to_string());
    }

    None
}

pub fn handle_bootstrap() {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    println!("Bootstrapping tinosRSE-Development...");

    println!("Installing Systempackages with Nix (llvm, mtools, grub2, limine, xorriso)...");

    let Some(nix_binary) = resolve_nix_binary() else {
        eprintln!("Warning: Could not find nix in PATH.");
        eprintln!("   Please install Nix first: https://nixos.org/download");
        return;
    };

    let nix_status = Command::new(&nix_binary)
        .args([
            "profile",
            "install",
            "nixpkgs#gcc",
            "nixpkgs#gnumake",
            "nixpkgs#llvm",
            "nixpkgs#mtools",
            "nixpkgs#grub2",
            "nixpkgs#limine",
            "nixpkgs#xorriso",
        ])
        .status();

    match nix_status {
        Ok(status) if status.success() => println!("Systempackages installed sucessfully."),
        _ => {
            eprintln!("Warning: Systempackage installation with nix failed or it was canceled.");
            eprintln!("   Make sure that Nix is installed and your internet connection is working.");
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

    sourceutils::fetch_source();

    if let (Ok(s1), Ok(s2), Ok(s3)) = (llvm_tools, rust_src, target_add) {
        if s1.success() && s2.success() && s3.success() {
            println!("All Rust-Components and x86_64-unknown-none installed!");
            println!("\n Environment is Ready! You can compile TinosRSE with the command `cargo run {prog} build`.");
            return;
        }
    }

    eprintln!("Error: Some Rust-components install failed.");
    eprintln!("   Make sure you have the latest nighly-toolchain");
}