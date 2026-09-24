use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn handle_build() -> PathBuf {
    println!("Compiling TinosRSE...");

    let mut child = Command::new("cargo")
        .args(["build", "--message-format=json"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute Cargo");

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut kernel_path: Option<PathBuf> = None;

    for line in reader.lines() {
        let line = line.expect("Failed to read Cargo output");

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
            if json["reason"] == "compiler-artifact"
                && json["target"]["name"] == "tinosrse"
            {
                if let Some(executable) = json["executable"].as_str() {
                    kernel_path = Some(PathBuf::from(executable));
                }
            }
        }

        // Keep Cargo's normal output visible
        println!("{}", line);
    }

    let status = child.wait().expect("Failed to wait for Cargo");

    if !status.success() {
        std::process::exit(1);
    }

    let kernel_path = kernel_path.expect("Cargo did not report the TinosRSE kernel");

    println!("Kernel: {}", kernel_path.display());

    kernel_path
}