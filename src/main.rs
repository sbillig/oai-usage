use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize)]
struct UsageResponse {
    #[allow(dead_code)]
    object: String,
    data: Vec<Bucket>,
    has_more: bool,
    next_page: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Bucket {
    // object: String,
    start_time: i64,
    // end_time: i64,
    results: Vec<UsageResult>,
}

#[derive(Debug, Deserialize)]
struct UsageResult {
    // object: String,
    input_tokens: u64,
    output_tokens: u64,
    num_model_requests: u64,
    model: Option<String>,
    input_cached_tokens: u64,
    // input_audio_tokens: u64,
    // output_audio_tokens: u64,
}

/// All values are in dollars per million tokens
#[derive(Debug, Clone)]
struct ModelPricing {
    input: f64,
    cached_input: f64,
    output: f64,
}

fn get_model_pricing() -> HashMap<String, ModelPricing> {
    let mut pricing = HashMap::new();

    pricing.insert(
        "gpt-5".to_string(),
        ModelPricing {
            input: 1.250,
            cached_input: 0.125,
            output: 10.000,
        },
    );

    pricing.insert(
        "gpt-5-mini".to_string(),
        ModelPricing {
            input: 0.250,
            cached_input: 0.025,
            output: 2.000,
        },
    );

    pricing.insert(
        "gpt-5-nano".to_string(),
        ModelPricing {
            input: 0.050,
            cached_input: 0.005,
            output: 0.400,
        },
    );

    pricing.insert(
        "o3".to_string(),
        ModelPricing {
            input: 2.00,
            cached_input: 0.50,
            output: 8.00,
        },
    );

    pricing.insert(
        "o4-mini".to_string(),
        ModelPricing {
            input: 1.10,
            cached_input: 0.275,
            output: 4.40,
        },
    );

    pricing.insert(
        "gpt-4.1".to_string(),
        ModelPricing {
            input: 2.00,
            cached_input: 0.50,
            output: 8.00,
        },
    );

    pricing.insert(
        "gpt-4.1-mini".to_string(),
        ModelPricing {
            input: 0.40,
            cached_input: 0.10,
            output: 1.60,
        },
    );

    pricing.insert(
        "gpt-4.1-nano".to_string(),
        ModelPricing {
            input: 0.10,
            cached_input: 0.025,
            output: 0.40,
        },
    );

    pricing
}

fn get_base_model_name(model_name: &str) -> &str {
    if model_name.starts_with("gpt-5-mini") {
        "gpt-5-mini"
    } else if model_name.starts_with("gpt-5-nano") {
        "gpt-5-nano"
    } else if model_name.starts_with("gpt-5") {
        "gpt-5"
    } else if model_name.starts_with("gpt-4.1-mini") {
        "gpt-4.1-mini"
    } else if model_name.starts_with("gpt-4.1-nano") {
        "gpt-4.1-nano"
    } else if model_name.starts_with("gpt-4.1") {
        "gpt-4.1"
    } else if model_name.starts_with("o4-mini") {
        "o4-mini"
    } else if model_name.starts_with("o3") {
        "o3"
    } else {
        model_name
    }
}

fn format_number(num: u64) -> String {
    let mut result = String::new();
    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}

fn calculate_cost(usage: &UsageResult, pricing: &ModelPricing) -> f64 {
    let input_tokens_non_cached = usage.input_tokens - usage.input_cached_tokens;
    let input_cost = (input_tokens_non_cached as f64 / 1_000_000.0) * pricing.input;
    let cached_cost = (usage.input_cached_tokens as f64 / 1_000_000.0) * pricing.cached_input;
    let output_cost = (usage.output_tokens as f64 / 1_000_000.0) * pricing.output;

    input_cost + cached_cost + output_cost
}

fn get_start_time_for_days_ago(days: u32) -> i64 {
    let now = Utc::now();
    let start = now - chrono::Duration::days(days as i64);
    start.timestamp()
}

fn fetch_usage_data(api_key: &str, start_time: i64) -> Result<UsageResponse> {
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

fn display_usage_table(
    response: &UsageResponse,
    pricing: &HashMap<String, ModelPricing>,
) -> Result<()> {
    let mut table = Table::new();
    table.set_header(vec![
        "Date",
        "Model",
        "Requests",
        "Input Tokens",
        "Cached Input",
        "Output Tokens",
        "Cost ($)",
    ]);

    let mut total_requests = 0u64;
    let mut total_input_tokens = 0u64;
    let mut total_cached_tokens = 0u64;
    let mut total_output_tokens = 0u64;
    let mut total_cost = 0.0f64;

    for bucket in &response.data {
        let date = DateTime::<Utc>::from_timestamp(bucket.start_time, 0)
            .context("Invalid timestamp")?
            .format("%Y-%m-%d")
            .to_string();

        for result in &bucket.results {
            let model_name = result.model.as_deref().unwrap_or("unknown");
            let base_model_name = get_base_model_name(model_name);
            let input_tokens_non_cached = result.input_tokens - result.input_cached_tokens;

            let cost = if let Some(model_pricing) = pricing.get(base_model_name) {
                calculate_cost(result, model_pricing)
            } else {
                0.0
            };

            total_requests += result.num_model_requests;
            total_input_tokens += input_tokens_non_cached;
            total_cached_tokens += result.input_cached_tokens;
            total_output_tokens += result.output_tokens;
            total_cost += cost;

            table.add_row(vec![
                Cell::new(&date),
                Cell::new(model_name),
                Cell::new(format_number(result.num_model_requests))
                    .set_alignment(CellAlignment::Right),
                Cell::new(format_number(input_tokens_non_cached))
                    .set_alignment(CellAlignment::Right),
                Cell::new(format_number(result.input_cached_tokens))
                    .set_alignment(CellAlignment::Right),
                Cell::new(format_number(result.output_tokens)).set_alignment(CellAlignment::Right),
                Cell::new(format!("{:.4}", cost)).set_alignment(CellAlignment::Right),
            ]);
        }
    }

    table.add_row(vec![
        Cell::new("TOTAL").add_attribute(Attribute::Bold),
        Cell::new("").add_attribute(Attribute::Bold),
        Cell::new(format_number(total_requests))
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
        Cell::new(format_number(total_input_tokens))
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
        Cell::new(format_number(total_cached_tokens))
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
        Cell::new(format_number(total_output_tokens))
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
        Cell::new(format!("{:.4}", total_cost))
            .set_alignment(CellAlignment::Right)
            .add_attribute(Attribute::Bold),
    ]);

    println!("{}", table);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let days = if args.len() > 1 {
        args[1]
            .parse::<u32>()
            .context("Invalid number of days. Please provide a positive integer.")?
    } else {
        3 // Default to 3 days if no argument provided
    };

    if days == 0 {
        anyhow::bail!("Number of days must be greater than 0");
    }

    let api_key =
        env::var("OPENAI_ADMIN_KEY").context("OPENAI_ADMIN_KEY environment variable not set")?;

    let start_time = get_start_time_for_days_ago(days);

    println!("Fetching OpenAI usage data for the past {} days...", days);

    let usage_response = fetch_usage_data(&api_key, start_time)?;
    let pricing = get_model_pricing();

    display_usage_table(&usage_response, &pricing)?;

    Ok(())
}
