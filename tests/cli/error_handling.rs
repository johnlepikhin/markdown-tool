use super::TestCommand;

#[test]
fn test_help_command() {
    let cmd = TestCommand::new();

    let output = cmd.run(&["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("convert-to"));
    assert!(stdout.contains("markdown-tool"));
}

#[test]
fn test_convert_to_help() {
    let cmd = TestCommand::new();

    let output = cmd.run(&["convert-to", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--from"));
    assert!(stdout.contains("markdown"));
    assert!(stdout.contains("html"));
    assert!(stdout.contains("latex"));
}

#[test]
fn test_invalid_input_format() {
    let cmd = TestCommand::new();

    let output = cmd.run(&["convert-to", "-f", "invalid-format", "markdown"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("format"));
}

#[test]
fn test_invalid_table_style() {
    let cmd = TestCommand::new();

    let output = cmd.run_with_input(
        &["convert-to", "latex", "--table-style", "invalid-style"],
        "# Test",
    );

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("style"));
}

#[test]
fn test_invalid_code_style() {
    let cmd = TestCommand::new();

    let output = cmd.run_with_input(
        &["convert-to", "latex", "--code-style", "invalid-style"],
        "# Test",
    );

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("style"));
}

#[test]
fn test_missing_subcommand() {
    let cmd = TestCommand::new();

    let output = cmd.run(&["convert-to", "-f", "markdown"]);

    assert!(!output.status.success());
}

#[test]
fn test_empty_input() {
    let cmd = TestCommand::new();

    let output = cmd.run_with_input(&["convert-to", "markdown"], "");

    // Empty input should be handled gracefully
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Output might be empty or minimal
    assert!(stdout.len() <= 10); // Should be very minimal
}

#[test]
fn test_large_input() {
    let cmd = TestCommand::new();

    // Create a large input (but not too large for tests)
    let large_input = "# Large Document\n\n".repeat(1000) + "Final paragraph.";

    let output = cmd.run_with_input(&["convert-to", "html"], &large_input);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<h1>"));
    assert!(stdout.contains("Final paragraph"));
}

#[test]
fn test_unicode_input() {
    let cmd = TestCommand::new();
    let unicode_input = "# Тест\n\nТекст с юникодом: 🚀 émojis ünicöde.";

    let output = cmd.run_with_input(&["convert-to", "html"], unicode_input);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("🚀"));
    assert!(stdout.contains("émojis"));
    assert!(stdout.contains("ünicöde"));
}

#[test]
fn test_malformed_markdown_graceful_handling() {
    let cmd = TestCommand::new();
    let malformed_input = "# Unclosed [link\n\n**Unclosed bold\n\n```\nUnclosed code block";

    let output = cmd.run_with_input(&["convert-to", "html"], malformed_input);

    // Should handle malformed markdown gracefully
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty());
}
