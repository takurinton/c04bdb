use std::env;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn image(ctx: &Context, msg: &Message) -> CommandResult {
    let arg = msg.content.clone();
    println!("arg: {}", arg);
    if arg != "" {
        let search_engine_id =
            env::var("SEARCH_ENGINE_ID").expect("search engine id is not defined");
        let api_key = env::var("API_KEY").expect("api key is not defined");
        let url = format!("https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&searchType=image&hl=ja&q={}", arg);
        let result = reqwest::get(&url).await?;
        let body = result.json::<serde_json::Value>().await?;
        let items = match body["items"].as_array() {
            Some(items) => items,
            None => return Ok(()),
        };
        let length = items.len();
        let random = rand::random::<usize>() % length;
        let item = &items[random];
        let link = match item["link"].as_str() {
            Some(link) => link,
            None => return Ok(()),
        };
        msg.channel_id.say(&ctx.http, link).await?;
    }
    Ok(())
}
