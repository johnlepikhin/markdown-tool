pub mod convert_to_ast;
pub mod convert_to_html;
pub mod convert_to_latex;
pub mod convert_to_markdown;
pub mod error_handling;

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct TestCommand {
    binary_path: String,
}

impl TestCommand {
    pub fn new() -> Self {
        let binary_path = env!("CARGO_BIN_EXE_markdown-tool").to_string();
        Self { binary_path }
    }

    pub fn run_with_input(&self, args: &[&str], input: &str) -> std::process::Output {
        let mut child = Command::new(&self.binary_path)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(input.as_bytes())
                .expect("Failed to write to stdin");
        }

        child.wait_with_output().expect("Failed to read output")
    }

    pub fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(&self.binary_path)
            .args(args)
            .output()
            .expect("Failed to execute command")
    }
}

pub fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file
}

pub fn assert_output_contains(output: &std::process::Output, expected: &str) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(expected),
        "Output does not contain expected text.\nExpected: {}\nActual: {}",
        expected,
        stdout
    );
}

pub fn assert_success(output: &std::process::Output) {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Command failed with stderr: {}", stderr);
    }
}
