use owo_colors::AnsiColors;
use regex::Regex;
use std::sync::LazyLock;

struct Rule {
    regex: Regex,
    color: AnsiColors,
}

static LOGS: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        Rule {
            regex: Regex::new(r"\bERROR\b").unwrap(),
            color: AnsiColors::Red,
        },
        Rule {
            regex: Regex::new(r"\bINFO\b").unwrap(),
            color: AnsiColors::White,
        },
        Rule {
            regex: Regex::new(r"\bWARNING\b").unwrap(),
            color: AnsiColors::Yellow,
        },
    ]
});

fn main() {
    println!("Hello, world!");
}
