[![crates.io](https://img.shields.io/crates/v/markdown-tool.svg)](https://crates.io/crates/markdown-tool)
[![docs.rs](https://docs.rs/markdown-tool/badge.svg)](https://docs.rs/markdown-tool)
[![CI](https://github.com/johnlepikhin/markdown-tool/actions/workflows/ci.yml/badge.svg)](https://github.com/johnlepikhin/markdown-tool/actions)
[![License: MIT](https://img.shields.io/crates/l/markdown-tool.svg)](https://github.com/johnlepikhin/markdown-tool/blob/main/LICENSE)


# markdown-tool

`markdown-tool` is a powerful command-line utility for converting Markdown documents to various formats including HTML, LaTeX, and abstract syntax tree (AST) representations.

## Features

- **Multiple Output Formats**: Convert Markdown to HTML, LaTeX, JSON-based AST, and YAML-based AST formats
- **LaTeX Support**: Generate LaTeX documents with configurable table styles and code highlighting
- **Configurable Output**: Customize formatting options for each output format
- **Lightweight CLI**: Zero-dependency binary, easily scriptable in CI/CD workflows
- **Round-trip Capable**: Parse and regenerate Markdown with consistent formatting

## Installation

Install via [crates.io](https://crates.io/crates/markdown-tool) (requires Rust and Cargo):

```bash
cargo install markdown-tool
```

Alternatively, download pre-built packages:

- Debian (`.deb`):
  ```bash
  sudo dpkg -i markdown-tool_<VERSION>_<ARCH>.deb
  ```

## Usage

The tool uses a modern subcommand structure for better organization:

```text
Usage: markdown-tool convert-to [OPTIONS] <COMMAND>

Commands:
  markdown  Convert to Markdown format
  html      Convert to HTML format
  latex     Convert to LaTeX format
  ast-json  Convert to AST JSON format
  ast-yaml  Convert to AST YAML format

Options:
  -f, --from <FROM>  Input format: markdown, ast-json, ast-yaml [default: markdown]
  -h, --help         Print help
```

### Quick Start

Most common use case (Markdown input is default):
```bash
echo "# Hello World" | markdown-tool convert-to html
# Output: <h1>Hello World</h1>
```

### Supported Formats

| Format     | Input | Output | Description                          |
| ---------- | ----- | ------ | ------------------------------------ |
| markdown   | ✅     | ✅      | Standard Markdown text               |
| html       | ❌     | ✅      | HTML-formatted text                  |
| latex      | ❌     | ✅      | LaTeX document format                |
| ast-json   | ✅     | ✅      | JSON-formatted abstract syntax tree  |
| ast-yaml   | ✅     | ✅      | YAML-formatted abstract syntax tree  |

## Examples

### Basic Conversions

Convert Markdown to HTML (default input format):
```bash
markdown-tool convert-to html < input.md > output.html
```

Convert Markdown to LaTeX with custom formatting:
```bash
markdown-tool convert-to latex --table-style booktabs --code-style minted < input.md > output.tex
```

Convert Markdown to JSON AST:
```bash
markdown-tool convert-to ast-json < input.md > ast.json
```

### Advanced Usage

Convert JSON AST back to Markdown:
```bash
markdown-tool convert-to -f ast-json markdown < ast.json > output.md
```

Convert YAML AST to Markdown with custom width:
```bash
markdown-tool convert-to -f ast-yaml markdown --width 120 < ast.yaml > output.md
```

Pretty-print Markdown with custom formatting:
```bash
markdown-tool convert-to markdown --width 100 --spaces-before-list-item 2 < unformatted.md > formatted.md
```

### Format-Specific Options

#### Markdown Output
```bash
markdown-tool convert-to markdown --width 120 --spaces-before-list-item 2 --no-empty-line-before-list
```

#### HTML Output
```bash
markdown-tool convert-to html --width 100 --anchor-prefix "section-"
```

#### LaTeX Output
```bash
markdown-tool convert-to latex --width 80 --table-style booktabs --code-style minted
```

Available LaTeX options:
- **Table styles**: `tabular` (default), `longtabu`, `booktabs`
- **Code styles**: `verbatim` (default), `listings`, `minted`

### LaTeX Package Requirements

Depending on the options used, you may need these LaTeX packages:

- `hyperref` - for links (`\href`)
- `graphicx` - for images (`\includegraphics`)
- `ulem` - for strikethrough (`\sout`)
- `booktabs` - if using `--table-style booktabs`
- `longtabu` - if using `--table-style longtabu`
- `listings` - if using `--code-style listings`
- `minted` - if using `--code-style minted` (requires Python + Pygments)

## 📚 Documentation

- [AI-generated documentation](https://deepwiki.com/johnlepikhin/markdown-tool)

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open an issue or submit a pull request on
[GitHub](https://github.com/johnlepikhin/markdown-tool).

## License

Dual-licensed under MIT. See [LICENSE](LICENSE) for details.

