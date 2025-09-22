use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct ConvertToAstYaml;

impl ConvertToAstYaml {
    pub fn run(&self, ast: &markdown_ppp::ast::Document) -> Result<()> {
        let result = serde_yaml::to_string(ast)?;
        println!("{result}");
        Ok(())
    }
}
