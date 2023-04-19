use reqwest::header;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn github_trend(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    let content = &msg.content;
    let content = content.replace("/github_trend ", "");
    let url = format!(
        "https://api.github.com/search/repositories?q={}&order=desc&per_page=10&since=daily",
        content
    );

    let client = reqwest::Client::new();
    let result = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "Rinton")
        .send();

    let result = match result.await {
        Ok(result) => result,
        Err(_) => {
            let _ = channel_id
                .say(&ctx.http, "通信エラーが発生しました。")
                .await;
            return Ok(());
        }
    };

    let _ = match result.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => {
            let _ = channel_id
                .say(&ctx.http, "unauthorized. 時間を置いてやり直してください。")
                .await;
            return Ok(());
        }
        reqwest::StatusCode::FORBIDDEN => {
            let _ = channel_id
                .say(&ctx.http, "forhidden. 時間を置いてやり直してください。")
                .await;
            return Ok(());
        }
        reqwest::StatusCode::NOT_FOUND => {
            let _ = channel_id
                .say(&ctx.http, "リポジトリが見つかりませんでした。")
                .await;
            return Ok(());
        }
        _ => {
            let _ = channel_id
                .say(
                    &ctx.http,
                    "予期しないエラーが発生しました。時間を置いてやり直してください。",
                )
                .await;
            return Ok(());
        }
    };

    let body = match result.json::<serde_json::Value>().await {
        Ok(body) => body,
        Err(_) => {
            let _ = channel_id
                .say(&ctx.http, "json の parse に失敗しました。")
                .await;
            return Ok(());
        }
    };

    let items = match body["items"].as_array() {
        Some(items) => items,
        None => {
            let _ = channel_id.say(&ctx.http, "レスポンスが空です。").await;
            return Ok(());
        }
    };

    for item in items {
        let name = match item["full_name"].as_str() {
            Some(name) => name,
            None => return Ok(()),
        };
        let avatar_url = match item["owner"]["avatar_url"].as_str() {
            Some(avatar_url) => avatar_url,
            None => return Ok(()),
        };
        let description = match item["description"].as_str() {
            Some(description) => description,
            None => return Ok(()),
        };
        let html_url = match item["html_url"].as_str() {
            Some(html_url) => html_url,
            None => return Ok(()),
        };
        let stars = match item["stargazers_count"].as_u64() {
            Some(stars) => stars,
            None => return Ok(()),
        };

        let _ = channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(name).url(html_url).icon_url(avatar_url);
                        a
                    });
                    e.description(description);
                    e.url(html_url);
                    e.field("Stars", stars, true);
                    e
                });
                m
            })
            .await;
    }
    Ok(())
}
