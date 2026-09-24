use std::process::Command;
use crate::commands::buildcmd;


pub fn handle_run(_gui: bool) {
    let target_dir = buildcmd::handle_build();

    let _bios_path = target_dir.join("debug/build"); 

    println!("Starting TinosRSE...");
    let mut binding = Command::new("cargo");
    let cargo = binding.args(&["run", "bios"]);
    
    

    let mut child = cargo.spawn().expect("QEMU startup failed");
    child.wait().unwrap();
}
