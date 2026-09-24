use std::process::Command;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use std::env;

pub fn fetch_source() {
    println!("Fetching TinosRSE source, please wait....");

    let mut binding = Command::new("git");
    let gitSource = binding.args(&["clone", "https://github.com/Tin-Systems-Platform/tinosrse"]);

    let mut child = gitSource.spawn().expect("Failed to fetch source code, do you have internet or do you even have git?");
    child.wait().unwrap();

    println!("Source code fetched");
}