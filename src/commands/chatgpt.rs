use std::env;

use reqwest;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct ChatGPTMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatGPTChoice {
    message: ChatGPTMessage,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<ChatGPTChoice>,
}

#[command]
async fn chatgpt(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = &msg.channel_id;
    let messages = msg.content.clone().replace("/chatgpt ", "");

    let typing = channel_id.start_typing(&ctx.http)?;

    let client = reqwest::Client::new();
    let response = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not defined"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(format!(
            r#"{{ "model": "gpt-3.5-turbo", "messages": [{{ "role": "user", "content": "{}" }}] }}"#,
            messages
        ))
        .send().await {
        Ok(response) => match response.json::<ChatGPTResponse>().await {
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
            format!("{}", response.choices[0].message.content.as_str()),
        )
        .await?;
    Ok(())
}
