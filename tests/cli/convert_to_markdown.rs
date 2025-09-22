use super::{assert_output_contains, assert_success, TestCommand};

#[test]
fn test_convert_to_markdown_basic() {
    let cmd = TestCommand::new();
    let input = "# Hello World\n\nThis is **bold** text.";

    let output = cmd.run_with_input(&["convert-to", "markdown"], input);

    assert_success(&output);
    assert_output_contains(&output, "# Hello World");
    assert_output_contains(&output, "**bold**");
}

#[test]
fn test_convert_to_markdown_with_width() {
    let cmd = TestCommand::new();
    let input = "This is a very long line that should be wrapped when using a specific width setting for the markdown output format.";

    let output = cmd.run_with_input(&["convert-to", "markdown", "--width", "50"], input);

    assert_success(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that lines are roughly within the specified width
    let lines: Vec<&str> = stdout.lines().collect();
    let has_wrapped = lines.iter().any(|line| line.len() <= 60); // Some tolerance
    assert!(has_wrapped, "Text should be wrapped to specified width");
}

#[test]
fn test_convert_to_markdown_with_list_options() {
    let cmd = TestCommand::new();
    let input = "- First item\n- Second item";

    let output = cmd.run_with_input(
        &[
            "convert-to",
            "markdown",
            "--spaces-before-list-item",
            "2",
            "--no-empty-line-before-list",
        ],
        input,
    );

    assert_success(&output);
    // Should still contain list items (exact formatting depends on implementation)
    assert_output_contains(&output, "First item");
    assert_output_contains(&output, "Second item");
}

#[test]
fn test_convert_ast_json_to_markdown() {
    let cmd = TestCommand::new();
    let input = r#"{"blocks":[{"Paragraph":[{"Text":"Hello world"}]}]}"#;

    let output = cmd.run_with_input(&["convert-to", "-f", "ast-json", "markdown"], input);

    assert_success(&output);
    assert_output_contains(&output, "Hello world");
}

#[test]
fn test_convert_markdown_roundtrip() {
    let cmd = TestCommand::new();
    let input = "# Test\n\n**Bold** and *italic* text.";

    // Convert to markdown (should be identity)
    let output = cmd.run_with_input(&["convert-to", "markdown"], input);

    assert_success(&output);
    let result = String::from_utf8_lossy(&output.stdout);

    // Should preserve essential structure
    assert!(result.contains("# Test"));
    assert!(result.contains("**Bold**"));
    assert!(result.contains("*italic*"));
}
