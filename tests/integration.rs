mod cli;

use cli::TestCommand;

#[test]
fn test_basic_workflow() {
    let cmd = TestCommand::new();

    // Test the most common workflow: markdown -> html
    let markdown = "# Welcome\n\nThis is a **test** document.";

    let output = cmd.run_with_input(&["convert-to", "html"], markdown);

    assert!(output.status.success());
    let result = String::from_utf8_lossy(&output.stdout);
    assert!(result.contains("<h1>Welcome</h1>"));
    assert!(result.contains("<b>test</b>") || result.contains("<strong>test</strong>"));
}

#[test]
fn test_format_chain() {
    let cmd = TestCommand::new();

    // Test markdown -> ast-json -> markdown chain
    let original = "# Test\n\nOriginal **content**.";

    // Convert to AST JSON
    let json_output = cmd.run_with_input(&["convert-to", "ast-json"], original);
    assert!(json_output.status.success());

    let json_str = String::from_utf8_lossy(&json_output.stdout);

    // Convert back to markdown
    let markdown_output =
        cmd.run_with_input(&["convert-to", "-f", "ast-json", "markdown"], &json_str);

    assert!(markdown_output.status.success());
    let result = String::from_utf8_lossy(&markdown_output.stdout);

    // Should preserve essential content
    assert!(result.contains("Test"));
    assert!(result.contains("content"));
    assert!(result.contains("**") || result.contains("*"));
}

#[test]
fn test_all_output_formats() {
    let cmd = TestCommand::new();
    let input = "# Test Document\n\nSimple **content** for testing.";

    let formats = vec![
        ("markdown", vec!["#", "**"]),
        ("html", vec!["<h1>", "<b>", "</b>"]),
        ("latex", vec!["\\section", "\\textbf"]),
        ("ast-json", vec!["{", "blocks"]),
        ("ast-yaml", vec!["blocks:", "-"]),
    ];

    for (format, expected_tokens) in formats {
        let output = cmd.run_with_input(&["convert-to", format], input);

        assert!(output.status.success(), "Format {} should succeed", format);

        let result = String::from_utf8_lossy(&output.stdout);
        assert!(
            !result.is_empty(),
            "Format {} should produce output",
            format
        );

        // Check that output contains expected format-specific tokens
        let has_tokens = expected_tokens.iter().any(|token| result.contains(token));
        assert!(
            has_tokens,
            "Format {} should contain expected tokens",
            format
        );
    }
}

#[test]
fn test_file_fixture_processing() {
    let cmd = TestCommand::new();

    // Read our test fixture
    let sample_content = std::fs::read_to_string("tests/fixtures/sample.md")
        .expect("Should be able to read sample.md fixture");

    // Test conversion to different formats
    let formats = ["html", "latex", "ast-json"];

    for format in &formats {
        let output = cmd.run_with_input(&["convert-to", format], &sample_content);

        assert!(
            output.status.success(),
            "Sample file conversion to {} should succeed",
            format
        );

        let result = String::from_utf8_lossy(&output.stdout);
        assert!(
            !result.is_empty(),
            "Sample file conversion to {} should produce output",
            format
        );

        // Should contain some content from the original
        assert!(
            result.to_lowercase().contains("sample")
                || result.to_lowercase().contains("document")
                || result.to_lowercase().contains("bold"),
            "Converted {} should contain original content",
            format
        );
    }
}
