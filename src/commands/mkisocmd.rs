use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;
use crate::commands::buildcmd;

pub fn handle_iso() {
    let target_dir = buildcmd::handle_build();
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