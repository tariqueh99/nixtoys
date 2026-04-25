use anyhow::{Ok, Result};

pub fn run(session_type: String) -> Result<()> {
    println!("Current Session Type: {session_type}");
    Ok(())
}
