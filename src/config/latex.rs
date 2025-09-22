use clap::Args;
use core::str::FromStr;

#[derive(Clone)]
pub enum TableStyle {
    Tabular,
    Longtabu,
    Booktabs,
}

impl FromStr for TableStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tabular" => Ok(TableStyle::Tabular),
            "longtabu" => Ok(TableStyle::Longtabu),
            "booktabs" => Ok(TableStyle::Booktabs),
            _ => Err(format!(
                "Invalid table style: {s}. Supported: tabular, longtabu, booktabs"
            )),
        }
    }
}

#[derive(Clone)]
pub enum CodeStyle {
    Verbatim,
    Listings,
    Minted,
}

impl FromStr for CodeStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "verbatim" => Ok(CodeStyle::Verbatim),
            "listings" => Ok(CodeStyle::Listings),
            "minted" => Ok(CodeStyle::Minted),
            _ => Err(format!(
                "Invalid code style: {s}. Supported: verbatim, listings, minted"
            )),
        }
    }
}

#[derive(Args)]
pub struct LatexConfig {
    /// Width for LaTeX output formatting
    #[clap(long, default_value_t = 80)]
    pub width: usize,

    /// Table style: tabular, longtabu, booktabs
    #[clap(long, default_value = "tabular")]
    pub table_style: TableStyle,

    /// Code block style: verbatim, listings, minted
    #[clap(long, default_value = "verbatim")]
    pub code_style: CodeStyle,
}

impl LatexConfig {
    pub fn to_printer_config(&self) -> markdown_ppp::latex_printer::config::Config {
        let table_style = match self.table_style {
            TableStyle::Tabular => markdown_ppp::latex_printer::config::TableStyle::Tabular,
            TableStyle::Longtabu => markdown_ppp::latex_printer::config::TableStyle::Longtabu,
            TableStyle::Booktabs => markdown_ppp::latex_printer::config::TableStyle::Booktabs,
        };

        let code_style = match self.code_style {
            CodeStyle::Verbatim => markdown_ppp::latex_printer::config::CodeBlockStyle::Verbatim,
            CodeStyle::Listings => markdown_ppp::latex_printer::config::CodeBlockStyle::Listings,
            CodeStyle::Minted => markdown_ppp::latex_printer::config::CodeBlockStyle::Minted,
        };

        markdown_ppp::latex_printer::config::Config::default()
            .with_width(self.width)
            .with_table_style(table_style)
            .with_code_block_style(code_style)
    }
}
