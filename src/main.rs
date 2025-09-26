mod commands;
mod config;
mod input;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Subcommand for the application
#[derive(Subcommand)]
enum CommandLine {
    /// Convert to various output formats
    #[clap(name = "convert-to")]
    ConvertTo(crate::commands::ConvertTo),
    /// Format markdown files
    Format(crate::commands::Format),
}

/// markdown-tool - a tool for converting markdown files to other formats
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Application {
    /// Subcommand
    #[clap(subcommand)]
    command: CommandLine,
}

impl Application {
    fn run_command(&self) -> Result<()> {
        match &self.command {
            CommandLine::ConvertTo(convert_to) => convert_to.run(),
            CommandLine::Format(format) => format.run(),
        }
    }

    pub fn run(&self) {
        if let Err(err) = self.run_command() {
            eprintln!("Failed with error: {err:#}");
            std::process::exit(1);
        }
    }
}

fn main() {
    Application::parse().run();
}
