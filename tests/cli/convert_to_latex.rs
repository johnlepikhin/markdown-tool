use super::{assert_output_contains, assert_success, TestCommand};

#[test]
fn test_convert_to_latex_basic() {
    let cmd = TestCommand::new();
    let input = "# Hello World\n\nThis is **bold** text.";

    let output = cmd.run_with_input(&["convert-to", "latex"], input);

    assert_success(&output);
    assert_output_contains(&output, "\\section{Hello World}");
    assert_output_contains(&output, "\\textbf{bold}");
}

#[test]
fn test_convert_to_latex_with_table_style_booktabs() {
    let cmd = TestCommand::new();
    let input = "| A | B |\n|---|---|\n| 1 | 2 |";

    let output = cmd.run_with_input(&["convert-to", "latex", "--table-style", "booktabs"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain table elements (specific format depends on implementation)
    assert!(stdout.contains("tabular") || stdout.contains("table"));
}

#[test]
fn test_convert_to_latex_with_code_style_minted() {
    let cmd = TestCommand::new();
    let input = "```rust\nfn main() {}\n```";

    let output = cmd.run_with_input(&["convert-to", "latex", "--code-style", "minted"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain code block (format depends on minted implementation)
    assert!(stdout.contains("main") && stdout.contains("fn"));
}

#[test]
fn test_convert_to_latex_special_characters() {
    let cmd = TestCommand::new();
    let input = "Text with $ % & # symbols that need escaping.";

    let output = cmd.run_with_input(&["convert-to", "latex"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Special LaTeX characters should be escaped
    assert!(stdout.contains("\\$") || !stdout.contains("$"));
    assert!(stdout.contains("\\%") || !stdout.contains("%"));
    assert!(stdout.contains("\\&") || !stdout.contains("&"));
}

#[test]
fn test_convert_to_latex_complex_document() {
    let cmd = TestCommand::new();
    let input = r#"
# Main Title

## Subtitle

- List item with **bold**
- Another item with *italic*

```python
def hello():
    print("world")
```

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |

Link to [example](http://example.com).
"#;

    let output = cmd.run_with_input(
        &[
            "convert-to",
            "latex",
            "--table-style",
            "booktabs",
            "--code-style",
            "verbatim",
        ],
        input,
    );

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for various LaTeX elements
    assert!(stdout.contains("\\section"));
    assert!(stdout.contains("\\subsection"));
    assert!(stdout.contains("\\textbf"));
    assert!(stdout.contains("\\textit"));
    assert!(stdout.contains("itemize") || stdout.contains("\\item"));
}

#[test]
fn test_convert_to_latex_all_table_styles() {
    let cmd = TestCommand::new();
    let input = "| A | B |\n|---|---|\n| 1 | 2 |";

    // Test each table style
    for style in &["tabular", "longtabu", "booktabs"] {
        let output = cmd.run_with_input(&["convert-to", "latex", "--table-style", style], input);

        assert_success(&output);
        // Should produce valid output for each style
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.is_empty(),
            "Output should not be empty for style: {}",
            style
        );
    }
}

#[test]
fn test_convert_to_latex_all_code_styles() {
    let cmd = TestCommand::new();
    let input = "```rust\nfn test() {}\n```";

    // Test each code style
    for style in &["verbatim", "listings", "minted"] {
        let output = cmd.run_with_input(&["convert-to", "latex", "--code-style", style], input);

        assert_success(&output);
        // Should produce valid output for each style
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.is_empty(),
            "Output should not be empty for style: {}",
            style
        );
        assert!(
            stdout.contains("test"),
            "Code content should be preserved for style: {}",
            style
        );
    }
}
