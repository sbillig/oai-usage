use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use std::collections::HashMap;
use std::env;

mod models;
use crate::models::{ModelPricing, base_model_name, model_pricing};
mod api;
use crate::api::{UsageResponse, UsageResult, fetch_usage_data};

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
    (Utc::now() - chrono::Duration::days(days as i64)).timestamp()
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
            let base_model_name = base_model_name(model_name);
            let input_tokens_non_cached = result.input_tokens - result.input_cached_tokens;

            let cost = pricing
                .get(base_model_name)
                .map(|p| calculate_cost(result, p))
                .unwrap_or(0.0);

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
    let days: u32 = env::args().nth(1).map_or(Ok(3), |s| {
        s.parse()
            .context("Invalid number of days. Please provide a positive integer.")
    })?;

    if days == 0 {
        anyhow::bail!("Number of days must be greater than 0");
    }

    let api_key =
        env::var("OPENAI_ADMIN_KEY").context("OPENAI_ADMIN_KEY environment variable not set")?;

    let start_time = get_start_time_for_days_ago(days);

    println!("Fetching OpenAI usage data for the past {} days...", days);

    let usage_response = fetch_usage_data(&api_key, start_time)?;
    let pricing = model_pricing();

    display_usage_table(&usage_response, &pricing)?;

    Ok(())
}
