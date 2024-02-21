use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::utils::google_search::google_search;
use crate::utils::http_client::HttpClient;
use crate::utils::percent_decode::percent_decode;

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

pub async fn run(options: &[CommandDataOption]) -> String {
    let search_text = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "検索に失敗しました。".to_string(),
            },
            None => return "検索に失敗しました。".to_string(),
        },
        None => return "クエリが見つかりませんでした。".to_string(),
    };

    let items = match google_search(search_text, "web", "ja.wikipedia.org").await {
        Ok(items) => items,
        Err(err) => return err,
    };

    let wikipedia = match items.iter().find(|item| match item.link.as_str() {
        link if link.starts_with("https://ja.wikipedia.org/wiki/") => true,
        _ => false,
    }) {
        Some(item) => item.link.clone(),
        None => return "wikipedia の検索に失敗しました。".to_string(),
    };

    let text = wikipedia.replace("https://ja.wikipedia.org/wiki/", "");
    let url = format!("https://ja.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}", text);
    let client = HttpClient::new();
    let response = match client.get(&url).await {
        Ok(response) => response,
        Err(_) => {
            return "通信エラーが発生しました".to_string();
        },
    };

    let json = match response.json::<Response>().await {
        Ok(json) => json,
        Err(_) => {
            return "jsonのparseに失敗しました".to_string();
        },
    };

    let res = format!(
        "検索クエリ: {}
検索結果: {}
{}
https://ja.wikipedia.org/wiki/{}",
        search_text,
        percent_decode(text.as_str()),
        json.query.pages.iter().next().unwrap().1.extract,
        json.query.pages.iter().next().unwrap().1.title
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
