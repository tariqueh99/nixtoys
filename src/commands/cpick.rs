use crate::cli::cpick::PickArgs;
use anyhow::{Ok, Result};

pub fn run(_args: PickArgs) -> Result<()> {
    println!("Hello from cpick");
    Ok(())
}
