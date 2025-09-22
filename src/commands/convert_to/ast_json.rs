use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertToAstJson;

impl ConvertToAstJson {
    pub fn run(&self, ast: &markdown_ppp::ast::Document) -> Result<()> {
        let result = serde_json::to_string(ast)?;
        println!("{result}");
        Ok(())
    }
}
