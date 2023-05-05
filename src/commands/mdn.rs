use reqwest;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use std::env;

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
        "https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&hl=ja&q={}+site:developer.mozilla.org",
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

    let url = match items.iter().find(|item| {
        item["link"]
            .as_str()
            .unwrap()
            .contains("developer.mozilla.org/ja")
    }) {
        Some(item) => item["link"].as_str().unwrap().to_string(),
        None => match items.get(0) {
            Some(item) => item["link"].as_str().unwrap().to_string(),
            None => {
                return "検索結果が見つかりませんでした。".to_string();
            }
        },
    };

    format!(
        "
検索文字列: {text}
{url}",
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("mdn")
        .description("developer.mozilla.org から検索して概要を返します。")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
