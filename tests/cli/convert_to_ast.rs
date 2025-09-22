use super::{assert_output_contains, assert_success, TestCommand};

#[test]
fn test_convert_to_ast_json() {
    let cmd = TestCommand::new();
    let input = "# Hello World\n\nThis is **bold** text.";

    let output = cmd.run_with_input(&["convert-to", "ast-json"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should be valid JSON
    assert!(stdout.starts_with("{"));
    assert!(stdout.trim().ends_with("}"));
    assert!(stdout.contains("blocks"));

    // Try to parse as JSON
    let json_result: serde_json::Result<serde_json::Value> = serde_json::from_str(&stdout);
    assert!(json_result.is_ok(), "Output should be valid JSON");
}

#[test]
fn test_convert_to_ast_yaml() {
    let cmd = TestCommand::new();
    let input = "# Hello World\n\nThis is **bold** text.";

    let output = cmd.run_with_input(&["convert-to", "ast-yaml"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain YAML structure
    assert!(stdout.contains("blocks:"));

    // Try to parse as YAML
    let yaml_result: serde_yaml::Result<serde_yaml::Value> = serde_yaml::from_str(&stdout);
    assert!(yaml_result.is_ok(), "Output should be valid YAML");
}

#[test]
fn test_ast_json_roundtrip() {
    let cmd = TestCommand::new();
    let input = "# Test\n\nSimple paragraph.";

    // Convert to JSON
    let json_output = cmd.run_with_input(&["convert-to", "ast-json"], input);
    assert_success(&json_output);

    let json_str = String::from_utf8_lossy(&json_output.stdout);

    // Convert back to markdown
    let markdown_output =
        cmd.run_with_input(&["convert-to", "-f", "ast-json", "markdown"], &json_str);
    assert_success(&markdown_output);

    let result = String::from_utf8_lossy(&markdown_output.stdout);
    assert!(result.contains("Test"));
    assert!(result.contains("Simple paragraph"));
}

#[test]
fn test_ast_yaml_roundtrip() {
    let cmd = TestCommand::new();
    let input = "# Test\n\nSimple paragraph.";

    // Convert to YAML
    let yaml_output = cmd.run_with_input(&["convert-to", "ast-yaml"], input);
    assert_success(&yaml_output);

    let yaml_str = String::from_utf8_lossy(&yaml_output.stdout);

    // Convert back to markdown
    let markdown_output =
        cmd.run_with_input(&["convert-to", "-f", "ast-yaml", "markdown"], &yaml_str);
    assert_success(&markdown_output);

    let result = String::from_utf8_lossy(&markdown_output.stdout);
    assert!(result.contains("Test"));
    assert!(result.contains("Simple paragraph"));
}

#[test]
fn test_convert_complex_markdown_to_ast() {
    let cmd = TestCommand::new();
    let input = r#"
# Title

## Subtitle

- Item 1 with **bold**
- Item 2 with *italic*

```rust
fn main() {
    println!("Hello");
}
```

| A | B |
|---|---|
| 1 | 2 |

[Link](http://example.com)
"#;

    // Test JSON output
    let json_output = cmd.run_with_input(&["convert-to", "ast-json"], input);
    assert_success(&json_output);

    let json_str = String::from_utf8_lossy(&json_output.stdout);
    let json_value: serde_json::Value =
        serde_json::from_str(&json_str).expect("Should be valid JSON");

    // Should have blocks array
    assert!(json_value.get("blocks").is_some());

    // Test YAML output
    let yaml_output = cmd.run_with_input(&["convert-to", "ast-yaml"], input);
    assert_success(&yaml_output);

    let yaml_str = String::from_utf8_lossy(&yaml_output.stdout);
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&yaml_str).expect("Should be valid YAML");

    // Should have blocks key
    assert!(yaml_value.get("blocks").is_some());
}

#[test]
fn test_invalid_ast_json_input() {
    let cmd = TestCommand::new();
    let invalid_json = "{invalid json}";

    let output = cmd.run_with_input(&["convert-to", "-f", "ast-json", "markdown"], invalid_json);

    // Should fail gracefully
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "Should provide error message for invalid JSON"
    );
}

#[test]
fn test_invalid_ast_yaml_input() {
    let cmd = TestCommand::new();
    let invalid_yaml = "invalid: yaml: structure: [unclosed";

    let output = cmd.run_with_input(&["convert-to", "-f", "ast-yaml", "markdown"], invalid_yaml);

    // Should fail gracefully
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.is_empty(),
        "Should provide error message for invalid YAML"
    );
}
