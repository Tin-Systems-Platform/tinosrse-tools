use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(name = "tinosrse-rools")]
#[command(bin_name = "tinosrse-rools")]
enum CargoCli {
    Tinos(TinosArgs),
}

#[derive(clap::Args)]
struct TinosArgs {
    #[command(subcommand)]
    command: TinosCommand,
}

#[derive(Subcommand)]
enum TinosCommand {
    Bootstrap,
    Build,
    Run {
        #[arg(long)]
        gui: bool,
    },
    Iso,
}

fn main() {
    // Don't allow windows execution at all. Due to some packages not having windows equivalents
    if cfg!(target_os = "windows") {
        eprintln!("Error: This tool doesn't support native Windows platform");
        eprintln!("   Please use WSL instead.");
        std::process::exit(1);
    }

    let CargoCli::Tinos(args) = CargoCli::parse();

    match args.command {
        TinosCommand::Bootstrap => handle_bootstrap(),
        TinosCommand::Build => { handle_build(); },
        TinosCommand::Run { gui } => handle_run(gui),
        TinosCommand::Iso => handle_iso(),
    }
}

fn handle_bootstrap() {
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

fn handle_build() -> PathBuf {
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

fn handle_run(gui: bool) {
    let target_dir = handle_build();

    let bios_path = target_dir.join("debug/build"); 

    println!("Starting TinosRSE...");
    let mut cargo = Command::new("cargo").args(&["run", "bios"]);
    
    

    let mut child = cargo.spawn().expect("QEMU startup failed");
    child.wait().unwrap();
}

fn handle_iso() {
    let target_dir = handle_build();
    println!("Creating bootable ISO-file...");

    Command::new("mkdir").arg("-p").arg("iso_root").status().unwrap();
    

    Command::new("cp").arg("-r").arg("haku_polku/bios.img").arg("iso_root/").status().unwrap();
    
    let status = Command::new("genisoimage")
        .args(&["-R", "-b", "bios.img", "-no-emul-boot", "-boot-load-size", "4", "-o", "tinosRSE.iso", "iso_root/"])
        .status()
        .expect("genisoimage-tool execution Failed. Is it installed (sudo apt install genisoimage)?");

    if status.success() {
        println!("ISO file created Successfully: tinosRSE.iso");
    }
}