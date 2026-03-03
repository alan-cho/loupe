use owo_colors::AnsiColors;
use regex::Regex;
use std::sync::LazyLock;

pub struct Rule {
    pub regex: Regex,
    pub color: AnsiColors,
}

pub struct Token {
    pub start_position: usize,
    pub end_position: usize,
    pub color: AnsiColors,
}

pub static LOGS: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        Rule {
            regex: Regex::new(r"(?i)\bERROR\b").unwrap(),
            color: AnsiColors::Red,
        },
        Rule {
            regex: Regex::new(r"(?i)\bINFO\b").unwrap(),
            color: AnsiColors::White,
        },
        Rule {
            regex: Regex::new(r"(?i)\bWARNING\b").unwrap(),
            color: AnsiColors::Yellow,
        },
    ]
});
