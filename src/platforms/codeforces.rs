use anyhow::Result;
use scraper::{Html, Selector};
use super::{Downloadable, Platform, TestCase};

pub struct Codeforces {
    client: reqwest::Client,
}

impl Codeforces {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }
}

impl Platform for Codeforces {
    fn detect(url: &str) -> bool {
        url.contains("codeforces.com")
    }
    
    fn name(&self) -> &str {
        "Codeforces"
    }
}

impl Downloadable for Codeforces {
    async fn download_test_cases(&self, url: &str) -> Result<Vec<TestCase>> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        let mut test_cases = Vec::new();
        
        // Codeforces test cases are in div.sample-test
        let input_selector = Selector::parse("div.input > pre").unwrap();
        let output_selector = Selector::parse("div.output > pre").unwrap();
        
        let inputs: Vec<String> = document.select(&input_selector)
            .map(|el| {
                // Remove the first line (usually contains "Input" or similar)
                let text = el.text().collect::<Vec<_>>().join("");
                text.lines().skip(0).collect::<Vec<_>>().join("\n")
            })
            .collect();
        
        let outputs: Vec<String> = document.select(&output_selector)
            .map(|el| {
                let text = el.text().collect::<Vec<_>>().join("");
                text.lines().skip(0).collect::<Vec<_>>().join("\n")
            })
            .collect();
        
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            test_cases.push(TestCase {
                input: input.trim().to_string() + "\n",
                output: output.trim().to_string() + "\n",
            });
        }
        
        Ok(test_cases)
    }
}
