use clap::Args;

#[derive(Args)]
pub struct HtmlConfig {
    /// Width for HTML output formatting
    #[clap(long, default_value_t = 80)]
    pub width: usize,

    /// Prefix for anchor links
    #[clap(long)]
    pub anchor_prefix: Option<String>,
}

impl HtmlConfig {
    pub fn to_printer_config(&self) -> markdown_ppp::html_printer::config::Config {
        let mut config =
            markdown_ppp::html_printer::config::Config::default().with_width(self.width);

        if let Some(ref prefix) = self.anchor_prefix {
            config = config.with_anchor_prefix(prefix.clone());
        }

        config
    }
}
