use anyhow::Result;
use core::str::FromStr;

#[derive(Clone)]
pub enum InputFormat {
    Markdown,
    AstJson,
    AstYaml,
}

impl FromStr for InputFormat {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" => Ok(InputFormat::Markdown),
            "ast-json" => Ok(InputFormat::AstJson),
            "ast-yaml" => Ok(InputFormat::AstYaml),
            _ => Err(format!(
                "Invalid input format: {s}. Supported formats: markdown, ast-json, ast-yaml"
            )),
        }
    }
}

impl InputFormat {
    pub fn parse(&self, input: &str) -> Result<markdown_ppp::ast::Document> {
        match self {
            InputFormat::Markdown => {
                let state = markdown_ppp::parser::MarkdownParserState::default();
                markdown_ppp::parser::parse_markdown(state, input)
                    .map_err(|err| anyhow::anyhow!("{err}"))
            }
            InputFormat::AstJson => {
                let doc: markdown_ppp::ast::Document = serde_json::from_str(input)?;
                Ok(doc)
            }
            InputFormat::AstYaml => {
                let doc: markdown_ppp::ast::Document = serde_yaml::from_str(input)?;
                Ok(doc)
            }
        }
    }
}
