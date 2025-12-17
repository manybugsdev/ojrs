use anyhow::Result;
use scraper::{Html, Selector};
use super::{Downloadable, Platform, TestCase};

pub struct AtCoder {
    client: reqwest::Client,
}

impl AtCoder {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }
}

impl Platform for AtCoder {
    fn detect(url: &str) -> bool {
        url.contains("atcoder.jp")
    }
    
    fn name(&self) -> &str {
        "AtCoder"
    }
}

impl Downloadable for AtCoder {
    async fn download_test_cases(&self, url: &str) -> Result<Vec<TestCase>> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        let mut test_cases = Vec::new();
        
        // AtCoder sample inputs are in <pre> tags with specific IDs
        let input_selector = Selector::parse("span.lang-en div.part > section > pre").unwrap();
        let inputs: Vec<String> = document.select(&input_selector)
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .collect();
        
        // Group inputs and outputs (they alternate)
        for i in (0..inputs.len()).step_by(2) {
            if i + 1 < inputs.len() {
                test_cases.push(TestCase {
                    input: inputs[i].clone(),
                    output: inputs[i + 1].clone(),
                });
            }
        }
        
        if test_cases.is_empty() {
            // Try alternative selector for older AtCoder format
            let section_selector = Selector::parse("div.part > section").unwrap();
            let h3_selector = Selector::parse("h3").unwrap();
            let pre_selector = Selector::parse("pre").unwrap();
            
            let mut current_input: Option<String> = None;
            
            for section in document.select(&section_selector) {
                if let Some(h3) = section.select(&h3_selector).next() {
                    let title = h3.text().collect::<String>();
                    if let Some(pre) = section.select(&pre_selector).next() {
                        let content = pre.text().collect::<String>();
                        
                        if title.contains("入力") || title.contains("Input") {
                            current_input = Some(content);
                        } else if (title.contains("出力") || title.contains("Output")) && current_input.is_some() {
                            test_cases.push(TestCase {
                                input: current_input.take().unwrap(),
                                output: content,
                            });
                        }
                    }
                }
            }
        }
        
        Ok(test_cases)
    }
}
