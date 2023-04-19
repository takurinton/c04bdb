use reqwest::header;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
struct Owner {
    avatar_url: String,
}

#[derive(Deserialize)]
struct GithubTrendItem {
    full_name: String,
    html_url: String,
    description: String,
    stargazers_count: u32,
    owner: Owner,
}

#[derive(Deserialize)]
struct GithubTrend {
    items: Vec<GithubTrendItem>,
}

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
    let response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "Rinton")
        .send();

    let response = match response.await {
        Ok(response) => response,
        Err(_) => {
            let _ = channel_id
                .say(&ctx.http, "通信エラーが発生しました。")
                .await;
            return Ok(());
        }
    };

    let _ = match response.status() {
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

    let body = match response.json::<GithubTrend>().await {
        Ok(body) => body,
        Err(e) => {
            println!("{:?}", e);
            let _ = channel_id
                .say(&ctx.http, "json の parse に失敗しました。")
                .await;
            return Ok(());
        }
    };

    for item in body.items {
        let name = item.full_name;
        let avatar_url = item.owner.avatar_url;
        let html_url = item.html_url;
        let description = item.description;
        let stars = item.stargazers_count;
        let _ = channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(name).url(html_url).icon_url(avatar_url);
                        a
                    });
                    e.description(description);
                    e.field("Stars", stars, true);
                    e
                });
                m
            })
            .await;
    }
    Ok(())
}
