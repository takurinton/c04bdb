use std::env;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

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
    let url = format!("https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&searchType=image&hl=ja&q={}", text);
    let result = match reqwest::get(&url).await {
        Ok(result) => result,
        Err(_) => return "画像が見つかりませんでした。".to_string(),
    };
    let body = match result.json::<serde_json::Value>().await {
        Ok(body) => body,
        Err(_) => return "画像が見つかりませんでした。".to_string(),
    };
    let items = match body["items"].as_array() {
        Some(items) => items,
        None => return "画像が見つかりませんでした。".to_string(),
    };
    let length = items.len();
    let random = rand::random::<usize>() % length;
    let item = &items[random];
    let link = match item["link"].as_str() {
        Some(link) => link,
        None => return "画像が見つかりませんでした。".to_string(),
    };

    format!(
        "検索クエリ:{}
{}",
        text, link
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("image")
        .description("Google画像検索の結果をランダムに返します")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
