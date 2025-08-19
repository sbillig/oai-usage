use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UsageResponse {
    #[allow(dead_code)]
    pub object: String,
    pub data: Vec<Bucket>,
    pub has_more: bool,
    pub next_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Bucket {
    // object: String,
    pub start_time: i64,
    // end_time: i64,
    pub results: Vec<UsageResult>,
}

#[derive(Debug, Deserialize)]
pub struct UsageResult {
    // object: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub num_model_requests: u64,
    pub model: Option<String>,
    pub input_cached_tokens: u64,
    // input_audio_tokens: u64,
    // output_audio_tokens: u64,
}

pub fn fetch_usage_data(api_key: &str, start_time: i64) -> Result<UsageResponse> {
    let client = reqwest::blocking::Client::new();
    let mut all_data = Vec::new();
    let mut next_page: Option<String> = None;
    let mut page_count = 0;

    loop {
        page_count += 1;
        if page_count > 1 {
            println!("Fetching page {}...", page_count);
        }

        let mut query_params = vec![
            ("start_time", start_time.to_string()),
            ("group_by", "model".to_string()),
            ("bucket_width", "1d".to_string()),
        ];

        if let Some(page) = &next_page {
            query_params.push(("page", page.clone()));
        }

        let response = client
            .get("https://api.openai.com/v1/organization/usage/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .query(&query_params)
            .send()
            .context("Failed to send request to OpenAI API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            anyhow::bail!("API request failed with status {}: {}", status, text);
        }

        let usage_response: UsageResponse =
            response.json().context("Failed to parse JSON response")?;

        all_data.extend(usage_response.data);

        if usage_response.has_more && usage_response.next_page.is_some() {
            next_page = usage_response.next_page;
        } else {
            break;
        }
    }

    if page_count > 1 {
        println!("Fetched {} pages total.", page_count);
    }

    Ok(UsageResponse {
        object: "page".to_string(),
        data: all_data,
        has_more: false,
        next_page: None,
    })
}

