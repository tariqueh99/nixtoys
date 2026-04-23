pub mod cpick;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nixtoys",
    about = "A milti-tool utility CLI",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Color picker tool
    #[command(name = "cpick")]
    ColorPicker(cpick::PickArgs),
}
