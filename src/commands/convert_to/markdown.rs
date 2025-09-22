use crate::config::MarkdownConfig;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertToMarkdown {
    #[clap(flatten)]
    pub config: MarkdownConfig,
}

impl ConvertToMarkdown {
    pub fn run(&self, ast: &markdown_ppp::ast::Document) -> Result<()> {
        let config = self.config.to_printer_config();
        let result = markdown_ppp::printer::render_markdown(ast, config);

        println!("{result}");
        Ok(())
    }
}
