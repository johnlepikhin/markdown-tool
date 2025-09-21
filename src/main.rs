mod convert;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Subcommand for the application
#[derive(Subcommand)]
enum CommandLine {
    /// Convert markdown to other formats and vice versa
    Convert(crate::convert::Convert),
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
            CommandLine::Convert(convert) => convert.run(),
        }
    }

    pub fn run(&self) {
        if let Err(err) = self.run_command() {
            eprintln!("Failed with error: {err:#}");
        }
    }
}

fn main() {
    Application::parse().run();
}
