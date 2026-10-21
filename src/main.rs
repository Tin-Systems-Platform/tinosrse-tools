mod commands;

use clap::{Parser, Subcommand};


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
    let CargoCli::Tinos(args) = CargoCli::parse();

    match args.command {
        TinosCommand::Bootstrap => commands::bootstrapcmd::handle_bootstrap(),
        TinosCommand::Build => { commands::buildcmd::handle_build(); },
        TinosCommand::Run { gui } => commands::runcmd::handle_run(gui),
        TinosCommand::Iso => commands::mkisocmd::handle_iso(),
    }
}
