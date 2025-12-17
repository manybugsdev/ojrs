use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Online Judge Tools in Rust"));
    assert!(stdout.contains("download"));
    assert!(stdout.contains("test"));
    assert!(stdout.contains("submit"));
    assert!(stdout.contains("login"));
}

#[test]
fn test_download_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "download", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Download test cases"));
}

#[test]
fn test_test_command_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "test", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Test your code"));
}

#[test]
fn test_platform_detection_atcoder() {
    use ojrs::platforms::detector::detect_platform;
    
    let result = detect_platform("https://atcoder.jp/contests/abc123/tasks/abc123_a");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name(), "AtCoder");
}

#[test]
fn test_platform_detection_codeforces() {
    use ojrs::platforms::detector::detect_platform;
    
    let result = detect_platform("https://codeforces.com/contest/1234/problem/A");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name(), "Codeforces");
}

#[test]
fn test_platform_detection_unsupported() {
    use ojrs::platforms::detector::detect_platform;
    
    let result = detect_platform("https://unsupported.com/problem");
    assert!(result.is_err());
}
