mod cli;
mod model;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Command};
use model::{parse_date, parse_time, OvertimeData};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load existing overtime data
    let mut data = OvertimeData::load().context("Failed to load overtime data")?;

    // Process the command
    match cli.command {
        Command::Add { time, date } => {
            let minutes = parse_time(&time)?;
            let date = parse_date(&date)?;

            data.add_overtime(date, minutes)?;

            println!(
                "Added {} minutes of overtime for {}",
                minutes,
                date.format("%d.%m.%Y")
            );
        }

        Command::Remove { time, date } => {
            let minutes = parse_time(&time)?;
            let date = parse_date(&date)?;

            data.remove_overtime(date, minutes)?;

            println!(
                "Removed {} minutes of overtime for {}",
                minutes,
                date.format("%d.%m.%Y")
            );
        }

        Command::List => {
            data.list_overtime()?;
        }
    }

    Ok(())
}
