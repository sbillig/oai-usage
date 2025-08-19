use std::collections::HashMap;

/// Prices are dollars per million tokens.
#[derive(Debug, Clone)]
pub struct ModelPricing {
    pub input: f64,
    pub cached_input: f64,
    pub output: f64,
}

pub fn model_pricing() -> HashMap<String, ModelPricing> {
    [
        (
            "gpt-5",
            ModelPricing {
                input: 1.250,
                cached_input: 0.125,
                output: 10.000,
            },
        ),
        (
            "gpt-5-mini",
            ModelPricing {
                input: 0.250,
                cached_input: 0.025,
                output: 2.000,
            },
        ),
        (
            "gpt-5-nano",
            ModelPricing {
                input: 0.050,
                cached_input: 0.005,
                output: 0.400,
            },
        ),
        (
            "o3",
            ModelPricing {
                input: 2.00,
                cached_input: 0.50,
                output: 8.00,
            },
        ),
        (
            "o4-mini",
            ModelPricing {
                input: 1.10,
                cached_input: 0.275,
                output: 4.40,
            },
        ),
        (
            "gpt-4.1",
            ModelPricing {
                input: 2.00,
                cached_input: 0.50,
                output: 8.00,
            },
        ),
        (
            "gpt-4.1-mini",
            ModelPricing {
                input: 0.40,
                cached_input: 0.10,
                output: 1.60,
            },
        ),
        (
            "gpt-4.1-nano",
            ModelPricing {
                input: 0.10,
                cached_input: 0.025,
                output: 0.40,
            },
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

pub fn base_model_name(model_name: &str) -> &str {
    for base in [
        "gpt-5-mini",
        "gpt-5-nano",
        "gpt-5",
        "gpt-4.1-mini",
        "gpt-4.1-nano",
        "gpt-4.1",
        "o4-mini",
        "o3",
    ] {
        if model_name.starts_with(base) {
            return base;
        }
    }
    model_name
}
