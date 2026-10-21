use std::process::Command;
use std::path::PathBuf;
use std::fs;

use crate::commands::buildcmd;

#[derive(Copy, Clone)]
enum Bootloader {
    Limine,
    Grub,
}

fn command_available(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn select_bootloader(limine_mkrescue_available: bool, grub_mkrescue_available: bool) -> Option<Bootloader> {
    if limine_mkrescue_available {
        Some(Bootloader::Limine)
    } else if grub_mkrescue_available {
        Some(Bootloader::Grub)
    } else {
        None
    }
}

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

    fs::write(
        "iso_root/boot/limine.conf",
        r#"
TIMEOUT=0
:TinosRSE
PROTOCOL=multiboot2
KERNEL_PATH=boot:///boot/tinosrse
"#,
    )
    .expect("Failed to create Limine configuration");

    let bootloader = select_bootloader(
        command_available("limine-mkrescue"),
        command_available("grub-mkrescue"),
    )
    .expect("Neither limine-mkrescue nor grub-mkrescue is installed.");

    let status = match bootloader {
        Bootloader::Limine => Command::new("limine-mkrescue")
            .args(["-o", "tinosRSE.iso", "iso_root"])
            .status()
            .expect("limine-mkrescue execution failed."),
        Bootloader::Grub => Command::new("grub-mkrescue")
            .args(["-o", "tinosRSE.iso", "iso_root"])
            .status()
            .expect("grub-mkrescue execution failed."),
    };

    if !status.success() {
        std::process::exit(1);
    }

    println!("ISO file created successfully: tinosRSE.iso");
}

#[cfg(test)]
mod tests {
    use super::{Bootloader, select_bootloader};

    #[test]
    fn prefers_limine_when_both_are_available() {
        let selected = select_bootloader(true, true);
        assert!(matches!(selected, Some(Bootloader::Limine)));
    }

    #[test]
    fn falls_back_to_grub_when_limine_is_missing() {
        let selected = select_bootloader(false, true);
        assert!(matches!(selected, Some(Bootloader::Grub)));
    }

    #[test]
    fn returns_none_when_no_bootloader_is_available() {
        let selected = select_bootloader(false, false);
        assert!(selected.is_none());
    }
}