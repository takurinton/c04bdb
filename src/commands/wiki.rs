use reqwest;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use std::env;

use serde::Deserialize;

#[derive(Deserialize)]
struct Pages {
    // pageid: i32,
    title: String,
    extract: String,
}

#[derive(Deserialize)]
struct Query {
    pages: std::collections::HashMap<String, Pages>,
}

#[derive(Deserialize)]
struct Response {
    query: Query,
}

fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("Invalid hex digit: {}", hex),
    }
}

fn percent_decode(input: &str) -> String {
    use std::str;

    let mut out = Vec::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let h = hex_to_u8(bytes[i + 1]);
                let l = hex_to_u8(bytes[i + 2]);
                out.push((h << 4) | l);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    str::from_utf8(&out).unwrap().to_string()
}

pub async fn run(options: &[CommandDataOption]) -> String {
    let text = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "検索に失敗しました。".to_string(),
            },
            None => return "検索に失敗しました。".to_string(),
        },
        None => return "クエリが見つかりませんでした。".to_string(),
    };

    let search_engine_id = env::var("SEARCH_ENGINE_ID").expect("search engine id is not defined");
    let api_key = env::var("API_KEY").expect("api key is not defined");
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&hl=ja&q={}+site:ja.wikipedia.org",
        text
    );
    let result = match reqwest::get(&url).await {
        Ok(result) => result,
        Err(_) => {
            return "Google 検索でエラーが発生しました。".to_string();
        }
    };
    let body = match result.json::<serde_json::Value>().await {
        Ok(body) => body,
        Err(_) => {
            return "Google 検索でエラーが発生しました。".to_string();
        }
    };
    let items = match body["items"].as_array() {
        Some(items) => items,
        None => {
            return "Google 検索でエラーが発生しました。".to_string();
        }
    };

    let wikipedia = match items.iter().find(|item| match item["link"].as_str() {
        Some(link) => link.starts_with("https://ja.wikipedia.org/wiki/"),
        None => false,
    }) {
        Some(item) => match item["link"].as_str() {
            Some(link) => link,
            None => return "wikipedia の取得に失敗しました。".to_string(),
        },
        None => {
            return "そのような項目はありません。".to_string();
        }
    };

    let text = wikipedia.replace("https://ja.wikipedia.org/wiki/", "");
    let client = reqwest::Client::new();
    let response = match client
        .get(format!("https://ja.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}", text))
        .send().await {
        Ok(response) => match response.json::<Response>().await {
            Ok(response) => response,
            Err(_) => {
                return "wikipedia の取得に失敗しました。".to_string();
            },
        },
        Err(_) => {
            return "通信エラーが発生しました".to_string();
        },
    };

    let res = format!(
        "検索クエリ: {}
{}
https://ja.wikipedia.org/wiki/{}",
        percent_decode(text.as_str()),
        response.query.pages.iter().next().unwrap().1.extract,
        response.query.pages.iter().next().unwrap().1.title
    );

    res
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("wiki")
        .description("Wikipedia から検索して概要を返します。")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
