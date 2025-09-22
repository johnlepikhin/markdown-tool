use crate::config::HtmlConfig;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertToHtml {
    #[clap(flatten)]
    pub config: HtmlConfig,
}

impl ConvertToHtml {
    pub fn run(&self, ast: &markdown_ppp::ast::Document) -> Result<()> {
        let config = self.config.to_printer_config();
        let result = markdown_ppp::html_printer::render_html(ast, config);

        println!("{result}");
        Ok(())
    }
}
