use clap::Args;

#[derive(Args)]
pub struct MarkdownConfig {
    /// Width for markdown output formatting
    #[clap(long, default_value_t = 80)]
    pub width: usize,

    /// Number of spaces before list items (0-3)
    #[clap(long, default_value_t = 1)]
    pub spaces_before_list_item: usize,

    /// Disable empty line before lists
    #[clap(long)]
    pub no_empty_line_before_list: bool,
}

impl MarkdownConfig {
    pub fn to_printer_config(&self) -> markdown_ppp::printer::config::Config {
        markdown_ppp::printer::config::Config::default()
            .with_width(self.width)
            .with_spaces_before_list_item(self.spaces_before_list_item)
            .with_empty_line_before_list(!self.no_empty_line_before_list)
    }
}
