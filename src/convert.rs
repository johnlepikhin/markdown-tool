use anyhow::Result;
use clap::Args;
use core::str::FromStr;
use std::io::Read;

#[derive(Clone)]
/// Document format for conversion. Possible values are: `markdown`, `ast-json`, and `ast-yaml`.
pub(crate) enum DocumentFormat {
    Markdown,
    AstJson,
    AstYaml,
}

impl FromStr for DocumentFormat {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" => Ok(DocumentFormat::Markdown),
            "ast-json" => Ok(DocumentFormat::AstJson),
            "ast-yaml" => Ok(DocumentFormat::AstYaml),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

/// Convert between different document formats. Input data is read from STDIN and output is written to STDOUT.
#[derive(Args)]
pub(crate) struct Convert {
    /// Inbound document format. Possible values are: `markdown`, `ast-json`, and `ast-yaml`.
    #[clap(long)]
    from: DocumentFormat,

    /// Outbound document format. Possible values are: `markdown`, `ast-json`, and `ast-yaml`.
    #[clap(long)]
    to: DocumentFormat,
}

impl Convert {
    fn read_inbound(&self) -> Result<String> {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        Ok(input)
    }

    fn parse_markdown(&self, input: &str) -> Result<markdown_ppp::ast::Document> {
        let state = markdown_ppp::parser::MarkdownParserState::default();
        let r = markdown_ppp::parser::parse_markdown(state, input)
            .map_err(|err| anyhow::anyhow!("{}", err.to_string()))?;
        Ok(r)
    }

    fn parse_ast_json(&self, input: &str) -> Result<markdown_ppp::ast::Document> {
        let r: markdown_ppp::ast::Document = serde_json::from_str(input)?;
        Ok(r)
    }

    fn parse_ast_yaml(&self, input: &str) -> Result<markdown_ppp::ast::Document> {
        let r: markdown_ppp::ast::Document = serde_yaml::from_str(input)?;
        Ok(r)
    }

    fn render_markdown(&self, doc: markdown_ppp::ast::Document) -> String {
        let config = markdown_ppp::printer::config::Config::default();
        markdown_ppp::printer::render_markdown(&doc, config)
    }

    fn render_ast_json(&self, doc: markdown_ppp::ast::Document) -> String {
        serde_json::to_string(&doc).unwrap()
    }

    fn render_ast_yaml(&self, doc: markdown_ppp::ast::Document) -> String {
        serde_yaml::to_string(&doc).unwrap()
    }

    pub(crate) fn run(&self) -> Result<()> {
        let input = self.read_inbound()?;
        let ast = match self.from {
            DocumentFormat::Markdown => self.parse_markdown(&input)?,
            DocumentFormat::AstJson => self.parse_ast_json(&input)?,
            DocumentFormat::AstYaml => self.parse_ast_yaml(&input)?,
        };

        let result = match self.to {
            DocumentFormat::Markdown => self.render_markdown(ast),
            DocumentFormat::AstJson => self.render_ast_json(ast),
            DocumentFormat::AstYaml => self.render_ast_yaml(ast),
        };

        println!("{}", result);

        Ok(())
    }
}
