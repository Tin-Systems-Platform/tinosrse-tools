use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;
use crate::commands::buildcmd;


pub fn handle_run(gui: bool) {
    let target_dir = buildcmd::handle_build();

    let bios_path = target_dir.join("debug/build"); 

    println!("Starting TinosRSE...");
    let mut binding = Command::new("cargo");
    let mut cargo = binding.args(&["run", "bios"]);
    
    

    let mut child = cargo.spawn().expect("QEMU startup failed");
    child.wait().unwrap();
}
