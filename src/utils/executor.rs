use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::io::Write;
use std::time::Duration;

pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration: Duration,
}

pub fn execute_with_input(command_str: &str, input: &str, _timeout_secs: u64) -> Result<ExecutionResult> {
    // TODO: Implement timeout functionality using wait_timeout or similar
    let start = std::time::Instant::now();
    
    // Parse command string
    let parts: Vec<&str> = command_str.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }
    
    let program = parts[0];
    let args = &parts[1..];
    
    // Create child process
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute command: {}", command_str))?;
    
    // Write input to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())
            .context("Failed to write to stdin")?;
    }
    
    // Wait for completion with timeout
    let output = child.wait_with_output()
        .context("Failed to wait for command")?;
    
    let duration = start.elapsed();
    
    Ok(ExecutionResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
        duration,
    })
}
