use crate::config::LatexConfig;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertToLatex {
    #[clap(flatten)]
    pub config: LatexConfig,
}

impl ConvertToLatex {
    pub fn run(&self, ast: &markdown_ppp::ast::Document) -> Result<()> {
        let config = self.config.to_printer_config();
        let result = markdown_ppp::latex_printer::render_latex(ast, config);

        println!("{result}");
        Ok(())
    }
}
