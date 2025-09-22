use super::{assert_output_contains, assert_success, TestCommand};

#[test]
fn test_convert_to_html_basic() {
    let cmd = TestCommand::new();
    let input = "# Hello World\n\nThis is **bold** text.";

    let output = cmd.run_with_input(&["convert-to", "html"], input);

    assert_success(&output);
    assert_output_contains(&output, "<h1>Hello World</h1>");
    assert_output_contains(&output, "<b>bold</b>");
}

#[test]
fn test_convert_to_html_with_anchor_prefix() {
    let cmd = TestCommand::new();
    let input = "# Section One\n\n## Subsection";

    let output = cmd.run_with_input(&["convert-to", "html", "--anchor-prefix", "doc-"], input);

    assert_success(&output);
    assert_output_contains(&output, "<h1>Section One</h1>");
    assert_output_contains(&output, "<h2>Subsection</h2>");
}

#[test]
fn test_convert_to_html_complex_elements() {
    let cmd = TestCommand::new();
    let input = r#"
# Title

- Item 1
- Item 2

```rust
fn main() {}
```

| A | B |
|---|---|
| 1 | 2 |

[Link](http://example.com)
"#;

    let output = cmd.run_with_input(&["convert-to", "html"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check various HTML elements
    assert!(stdout.contains("<h1>"));
    assert!(stdout.contains("<ul>") || stdout.contains("<li>"));
    assert!(stdout.contains("<code>") || stdout.contains("<pre>"));
    assert!(stdout.contains("<table>") || stdout.contains("<td>"));
    assert!(stdout.contains("<a href="));
}

#[test]
fn test_convert_to_html_escaping() {
    let cmd = TestCommand::new();
    let input = "Text with <script> and & symbols.";

    let output = cmd.run_with_input(&["convert-to", "html"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // HTML entities should be escaped
    assert!(stdout.contains("&lt;") || stdout.contains("script"));
    assert!(stdout.contains("&amp;") || stdout.contains("&"));
}

#[test]
fn test_convert_ast_json_to_html() {
    let cmd = TestCommand::new();
    let input = r#"{"blocks":[{"Heading":{"kind":{"Atx":1},"content":[{"Text":"Title"}]}}]}"#;

    let output = cmd.run_with_input(&["convert-to", "-f", "ast-json", "html"], input);

    assert_success(&output);
    assert_output_contains(&output, "<h1>Title</h1>");
}
