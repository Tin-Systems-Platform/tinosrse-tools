# TinosRSE tools
TinosRSE tools is a CLI tool suite for compiling [TinosRSE](https://github.com/Tin-Systems-Platform/tinosrse).

## Usage
This tool is primararely used for TinosRSE. You can use this for your own things, but this is a custom made tool for the OS.
### Installing
This tool is available on crates.io.

install using cargo doing followingly:
```shell
cargo install tinosrse-tools
```

Or set up from source:
```shell
git clone https://github.com/Tin-Systems-Platform/tinosrse-tools # Clone the repository using GIT
cd tinosrse-tools # go to the cloned folder
cargo build # Build the source
cargo install --path . # install from source
```

### Requirements
The platform requirement is very strict as of now and will probably grow as this gets developed.

For now though following is the requirements:
OS: Linux with the APT package manager
Rust, cargo.

### Using the tool
To start bootstrapping do the following:
```shell
tinosrse-tools tinos bootstrap # This will make sure everything is installed
```

To build TinosRSE:
```shell
tinosrse-tools tinos build
```

List all of the sub commands:
```shell
tinosrse-tools tinos
```