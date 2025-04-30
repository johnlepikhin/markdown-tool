# markdown-tool

`markdown-tool` is a simple command-line utility for converting Markdown documents to various formats and abstract
syntax tree (AST) representations, and vice versa.

## Features

- Convert between Markdown, JSON-based AST, and YAML-based AST formats
- Lightweight, zero-dependency CLI
- Easily scriptable in CI/CD workflows or shell scripts

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
- RPM (`.rpm`):
  ```bash
  sudo rpm -i markdown-tool-<VERSION>.<ARCH>.rpm
  ```

## Usage

```text
Usage: markdown-tool convert --from <FROM> --to <TO>

Options:
      --from <FROM>  Inbound document format. Possible values are: `markdown`, `ast-json`, and `ast-yaml`
      --to <TO>      Outbound document format. Possible values are: `markdown`, `ast-json`, and `ast-yaml`
```

### Supported Formats

| Key        | Description                         |
| ---------- | ----------------------------------- |
| markdown   | Standard Markdown text              |
| ast-json   | JSON-formatted abstract syntax tree |
| ast-yaml   | YAML-formatted abstract syntax tree |

## Examples

Convert a Markdown file to JSON AST:

```bash
markdown-tool convert --from markdown --to ast-json < input.md > ast.json
```

Convert JSON AST back to Markdown:

```bash
markdown-tool convert --from ast-json --to markdown < ast.json > output.md
```

Convert YAML AST to Markdown:

```bash
markdown-tool convert --from ast-yaml --to markdown < ast.yaml > output.md
```

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open an issue or submit a pull request on
[GitHub](https://github.com/johnlepikhin/markdown-tool).

## License

Dual-licensed under MIT. See [LICENSE](LICENSE) for details.

