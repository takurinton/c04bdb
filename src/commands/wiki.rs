use reqwest;
use std::env;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

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

#[command]
async fn wiki(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = &msg.channel_id;
    let text = msg.content.clone().replace("~wiki ", "");
    // let text = to_parcent_encoding(text);

    let typing = channel_id.start_typing(&ctx.http)?;

    let search_engine_id = env::var("SEARCH_ENGINE_ID").expect("search engine id is not defined");
    let api_key = env::var("API_KEY").expect("api key is not defined");
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&hl=ja&q={}+site:ja.wikipedia.org",
        text
    );
    let result = reqwest::get(&url).await?;
    let body = result.json::<serde_json::Value>().await?;
    let items = match body["items"].as_array() {
        Some(items) => items,
        None => {
            msg.channel_id
                .say(&ctx.http, "Google 検索でエラーが発生しました。")
                .await?;
            return Ok(());
        }
    };

    let wikipedia = match items.iter().find(|item| match item["link"].as_str() {
        Some(link) => link.starts_with("https://ja.wikipedia.org/wiki/"),
        None => false,
    }) {
        Some(item) => match item["link"].as_str() {
            Some(link) => link,
            None => return Ok(()),
        },
        None => {
            msg.channel_id
                .say(&ctx.http, "そのような項目はありません。")
                .await?;
            return Ok(());
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
                msg.channel_id
                    .say(&ctx.http, "wikipedia の取得に失敗しました。")
                    .await?;
                return Ok(());
            },
        },
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "通信エラーが発生しました。")
                .await?;
            return Ok(());
        },
    };

    let _ = typing.stop();
    let res = format!(
        "{}
https://ja.wikipedia.org/wiki/{}",
        response.query.pages.iter().next().unwrap().1.extract,
        response.query.pages.iter().next().unwrap().1.title
    );

    msg.channel_id.say(&ctx.http, res).await?;
    Ok(())
}
