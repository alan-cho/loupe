use owo_colors::{AnsiColors, OwoColorize};
use regex::Regex;
use std::io::{self, BufRead};
use std::sync::LazyLock;

struct Rule {
    regex: Regex,
    color: AnsiColors,
}

struct Token {
    start_position: usize,
    end_position: usize,
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
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        match_rules(&LOGS, line);
    }
}

fn match_rules(rules: &Vec<Rule>, line: String) {
    let mut tokens: Vec<Token> = Vec::new();
    let colorized = line;
    for rule in rules.iter() {
        for m in rule.regex.find_iter(&colorized) {
            tokens.push(Token {
                start_position: m.start(),
                end_position: m.end(),
                color: rule.color,
            })
        }
    }

    tokens.sort_by_key(|t| t.start_position);
    let mut cursor: usize = 0;
    for token in &tokens {
        print!("{}", &colorized[cursor..token.start_position]);
        print!(
            "{}",
            (&colorized[token.start_position..token.end_position]).color(token.color)
        );
        cursor = token.end_position;
    }
    println!("{}", &colorized[cursor..]);
}
