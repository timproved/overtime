use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add overtime for a specific date
    Add {
        /// Overtime duration in format like 1h30m
        time: String,

        /// Date in format ddmmYYYY or dd.mm.YYYY
        date: String,
    },

    /// Remove overtime for a specific date
    Remove {
        /// Overtime duration in format like 1h30m
        time: String,

        /// Date in format ddmmYYYY or dd.mm.YYYY
        date: String,
    },

    /// List all overtime entries with a summary
    List,
}
