use reqwest;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Pages {
    // pageid: i32,
    // title: String,
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
    let text = msg.content.clone().replace("/wiki ", "");

    let typing = channel_id.start_typing(&ctx.http)?;

    let client = reqwest::Client::new();
    let response = match client
        .get(format!("https://ja.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}", text))
        .send().await {
        Ok(response) => match response.json::<Response>().await {
            Ok(response) => response,
            Err(_) => {
                msg.channel_id
                    .say(&ctx.http, "通信エラーが発生しました。")
                    .await?;
                return Ok(());
            },
        },
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "コンテンツの取得に失敗しました。")
                .await?;
            return Ok(());
        },
    };

    let _ = typing.stop();

    msg.channel_id
        .say(
            &ctx.http,
            format!("{}", response.query.pages.iter().next().unwrap().1.extract),
        )
        .await?;
    Ok(())
}
