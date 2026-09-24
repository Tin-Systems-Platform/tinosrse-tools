use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;
use std::fs;

use crate::commands::buildcmd;

pub fn handle_iso() {
    let kernel_path = buildcmd::handle_build();

    println!("Creating bootable ISO-file...");

    let grub_dir = PathBuf::from("iso_root/boot/grub");

    fs::create_dir_all(&grub_dir)
        .expect("Failed to create ISO directory structure");

    fs::copy(
        &kernel_path,
        "iso_root/boot/tinosrse",
    )
    .expect("Failed to copy kernel into ISO");

    fs::write(
        "iso_root/boot/grub/grub.cfg",
        r#"
menuentry "TinosRSE" {
    multiboot2 /boot/tinosrse
    boot
}
"#,
    )
    .expect("Failed to create GRUB configuration");

    let status = Command::new("grub-mkrescue")
        .args(["-o", "tinosRSE.iso", "iso_root"])
        .status()
        .expect(
            "grub-mkrescue execution failed. \
             Is GRUB installed?"
        );

    if !status.success() {
        std::process::exit(1);
    }

    println!("ISO file created successfully: tinosRSE.iso");
}