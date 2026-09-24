use std::process::Command;

pub fn fetch_source() {
    println!("Fetching TinosRSE source, please wait....");

    let mut binding = Command::new("git");
    let git_source = binding.args(&["clone", "https://github.com/Tin-Systems-Platform/tinosrse"]);

    let mut child = git_source.spawn().expect("Failed to fetch source code, do you have internet or do you even have git?");
    child.wait().unwrap();

    println!("Source code fetched");
}