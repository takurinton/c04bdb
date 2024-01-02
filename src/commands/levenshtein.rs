use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut dp = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    for i in 0..=a_chars.len() {
        dp[i][0] = i;
    }
    for j in 0..=b_chars.len() {
        dp[0][j] = j;
    }

    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[a_chars.len()][b_chars.len()]
}

#[test]
fn test_levenshtein() {
    assert_eq!(levenshtein("a", "a"), 0);
    assert_eq!(levenshtein("a", "b"), 1);
    assert_eq!(levenshtein("kitten", "sitting"), 3);
    assert_eq!(levenshtein("あいうえお", "あいうえお"), 0);
    assert_eq!(levenshtein("ねこ", "いぬ"), 2);
    assert_eq!(levenshtein("こんにちは", "こんばんは"), 2);
}

pub fn run(options: &[CommandDataOption]) -> String {
    let a = match options.iter().find(|option| option.name == "a") {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "a が不正です".to_string(),
            },
            None => return "a が不正です".to_string(),
        },
        None => return "a が不正です".to_string(),
    };

    let b = match options.iter().find(|option| option.name == "b") {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "b が不正です".to_string(),
            },
            None => return "b が不正です".to_string(),
        },
        None => return "b が不正です".to_string(),
    };

    levenshtein(a, b).to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("levenshtein")
        .description("2つの文字列を比較します。")
        .create_option(|option| {
            option
                .name("a")
                .description("比較する文字1")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("b")
                .description("比較する文字2")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
