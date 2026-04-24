mod cli;
mod commands;

use anyhow::Ok;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ColorPicker(args) => commands::cpick::run(args)?,
        Commands::Doctor => commands::doctor::run()?,
    }

    Ok(())
}
