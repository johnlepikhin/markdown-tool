use crate::config::MarkdownConfig;
use crate::input::InputFormat;
use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct Format {
    #[clap(flatten)]
    pub config: MarkdownConfig,

    /// Check if files need formatting without modifying them
    #[clap(long, short = 'n')]
    pub dry_run: bool,

    /// Files to format
    #[clap(required = true)]
    pub files: Vec<PathBuf>,
}

impl Format {
    pub fn run(&self) -> Result<()> {
        let mut needs_formatting = false;

        for file_path in &self.files {
            let original_content = fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

            let ast = InputFormat::Markdown
                .parse(&original_content)
                .with_context(|| format!("Failed to parse file: {}", file_path.display()))?;

            let printer_config = self.config.to_printer_config();
            let mut formatted_content =
                markdown_ppp::printer::render_markdown(&ast, printer_config);

            // Ensure the formatted content ends with a newline if the original did
            if original_content.ends_with('\n') && !formatted_content.ends_with('\n') {
                formatted_content.push('\n');
            }

            if original_content != formatted_content {
                needs_formatting = true;
                if self.dry_run {
                    println!("File needs formatting: {}", file_path.display());
                } else {
                    self.write_file_atomically(file_path, &formatted_content)
                        .with_context(|| {
                            format!("Failed to write file: {}", file_path.display())
                        })?;
                    println!("Formatted: {}", file_path.display());
                }
            }
        }

        if self.dry_run {
            if needs_formatting {
                std::process::exit(1);
            } else {
                println!("All files are already formatted");
            }
        }

        Ok(())
    }

    fn write_file_atomically(&self, file_path: &PathBuf, content: &str) -> Result<()> {
        let temp_file_path = file_path.with_extension(format!(
            "{}.tmp",
            file_path.extension().unwrap_or_default().to_string_lossy()
        ));

        fs::write(&temp_file_path, content).with_context(|| {
            format!(
                "Failed to write temporary file: {}",
                temp_file_path.display()
            )
        })?;

        fs::rename(&temp_file_path, file_path).with_context(|| {
            format!(
                "Failed to rename temporary file to: {}",
                file_path.display()
            )
        })?;

        Ok(())
    }
}
