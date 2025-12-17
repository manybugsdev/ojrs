use anyhow::Result;
use crate::utils::{executor, file_system, output};
use colored::Colorize;

pub fn execute(command: &str, directory: &str) -> Result<()> {
    output::info(&format!("Testing with command: {}", command));
    
    if !file_system::file_exists(directory) {
        anyhow::bail!("Test directory does not exist: {}", directory);
    }
    
    // Find all input files
    let input_files = file_system::list_files(directory, "in")?;
    
    if input_files.is_empty() {
        output::warning(&format!("No test cases found in {}", directory));
        return Ok(());
    }
    
    output::info(&format!("Found {} test case(s)", input_files.len()));
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    for input_file in input_files.iter() {
        let test_number = extract_test_number(input_file);
        let output_file = input_file.replace(".in", ".out");
        
        if !file_system::file_exists(&output_file) {
            output::warning(&format!("Test {}: Missing expected output file", test_number));
            continue;
        }
        
        let input = file_system::read_file(input_file)?;
        let expected_output = file_system::read_file(&output_file)?;
        
        print!("Test {} ... ", test_number);
        
        match executor::execute_with_input(command, &input, 10) {
            Ok(result) => {
                if result.exit_code != 0 {
                    println!("{}", "RE".red().bold());
                    println!("  Exit code: {}", result.exit_code);
                    if !result.stderr.is_empty() {
                        println!("  Stderr: {}", result.stderr.trim());
                    }
                    failed += 1;
                } else {
                    let actual_output = normalize_output(&result.stdout);
                    let expected = normalize_output(&expected_output);
                    
                    if actual_output == expected {
                        println!("{}", "AC".green().bold());
                        println!("  Time: {:?}", result.duration);
                        passed += 1;
                    } else {
                        println!("{}", "WA".red().bold());
                        println!("  Expected:");
                        for line in expected.lines().take(5) {
                            println!("    {}", line);
                        }
                        if expected.lines().count() > 5 {
                            println!("    ...");
                        }
                        println!("  Got:");
                        for line in actual_output.lines().take(5) {
                            println!("    {}", line);
                        }
                        if actual_output.lines().count() > 5 {
                            println!("    ...");
                        }
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                println!("{}", "ERROR".red().bold());
                println!("  {}", e);
                failed += 1;
            }
        }
        println!();
    }
    
    println!("Results: {} passed, {} failed", 
        passed.to_string().green().bold(), 
        failed.to_string().red().bold());
    
    if failed == 0 {
        output::success("All tests passed!");
    } else {
        output::error(&format!("{} test(s) failed", failed));
    }
    
    Ok(())
}

fn extract_test_number(filename: &str) -> String {
    filename.split('/').last()
        .unwrap_or(filename)
        .replace(".in", "")
        .replace("sample-", "")
}

fn normalize_output(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_output() {
        assert_eq!(normalize_output("hello\n"), "hello");
        assert_eq!(normalize_output("hello  \n"), "hello");
        assert_eq!(normalize_output("hello\nworld\n"), "hello\nworld");
        assert_eq!(normalize_output("  hello  \n  world  \n"), "hello\n  world");
    }

    #[test]
    fn test_extract_test_number() {
        assert_eq!(extract_test_number("test/sample-1.in"), "1");
        assert_eq!(extract_test_number("test/sample-10.in"), "10");
        assert_eq!(extract_test_number("sample-2.in"), "2");
    }
}
