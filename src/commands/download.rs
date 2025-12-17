use anyhow::Result;
use crate::platforms::{detector, Downloadable};
use crate::utils::{file_system, output};

pub async fn execute(url: &str, directory: &str) -> Result<()> {
    output::info(&format!("Downloading test cases from: {}", url));
    
    let platform = detector::detect_platform(url)?;
    output::info(&format!("Detected platform: {}", platform.name()));
    
    let test_cases = match platform {
        detector::PlatformType::AtCoder(ref p) => p.download_test_cases(url).await?,
        detector::PlatformType::Codeforces(ref p) => p.download_test_cases(url).await?,
    };
    
    if test_cases.is_empty() {
        output::warning("No test cases found!");
        return Ok(());
    }
    
    output::info(&format!("Found {} test case(s)", test_cases.len()));
    
    // Create directory if it doesn't exist
    file_system::create_directory(directory)?;
    
    // Save test cases
    for (i, test_case) in test_cases.iter().enumerate() {
        let input_file = format!("{}/sample-{}.in", directory, i + 1);
        let output_file = format!("{}/sample-{}.out", directory, i + 1);
        
        file_system::write_file(&input_file, &test_case.input)?;
        file_system::write_file(&output_file, &test_case.output)?;
        
        output::success(&format!("Saved test case {} to {} and {}", 
            i + 1, input_file, output_file));
    }
    
    output::success(&format!("Downloaded {} test case(s) successfully!", test_cases.len()));
    
    Ok(())
}
