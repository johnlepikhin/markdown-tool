pub mod ast_json;
pub mod ast_yaml;
pub mod html;
pub mod latex;
pub mod markdown;

use crate::input::{read_input, InputFormat};
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ConvertTo {
    /// Input format: markdown, ast-json, ast-yaml
    #[clap(long, short = 'f', default_value = "markdown")]
    pub from: InputFormat,

    #[clap(subcommand)]
    pub output: OutputFormat,
}

#[derive(Subcommand)]
pub enum OutputFormat {
    /// Convert to Markdown format
    Markdown(markdown::ConvertToMarkdown),
    /// Convert to HTML format
    Html(html::ConvertToHtml),
    /// Convert to LaTeX format
    Latex(latex::ConvertToLatex),
    /// Convert to AST JSON format
    #[clap(name = "ast-json")]
    AstJson(ast_json::ConvertToAstJson),
    /// Convert to AST YAML format
    #[clap(name = "ast-yaml")]
    AstYaml(ast_yaml::ConvertToAstYaml),
}

impl ConvertTo {
    pub fn run(&self) -> anyhow::Result<()> {
        let input_text = read_input()?;
        let ast = self.from.parse(&input_text)?;

        match &self.output {
            OutputFormat::Markdown(cmd) => cmd.run(&ast),
            OutputFormat::Html(cmd) => cmd.run(&ast),
            OutputFormat::Latex(cmd) => cmd.run(&ast),
            OutputFormat::AstJson(cmd) => cmd.run(&ast),
            OutputFormat::AstYaml(cmd) => cmd.run(&ast),
        }
    }
}
