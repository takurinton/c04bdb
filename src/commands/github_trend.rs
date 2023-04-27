use reqwest::header;
use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::prelude::*;

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

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> String {
    let options = &command.data.options;
    let language = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "言語を指定してください".to_string(),
            },
            None => return "言語を指定してください".to_string(),
        },
        None => return "言語を指定してください".to_string(),
    };

    let url = format!("https://api.github.com/search/repositories?q=language:{language}&order=desc&per_page=10&since=daily");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "Rinton")
        .send();

    let response = match response.await {
        Ok(response) => response,
        Err(_) => return "通信エラーが発生しました。".to_string(),
    };

    let _ = match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => {
            return "unauthorized. 時間を置いてやり直してください。".to_string()
        }
        reqwest::StatusCode::FORBIDDEN => {
            return "forhidden. 時間を置いてやり直してください。".to_string()
        }
        reqwest::StatusCode::NOT_FOUND => return "リポジトリが見つかりませんでした。".to_string(),
        _ => return "予期しないエラーが発生しました。時間を置いてやり直してください。".to_string(),
    };

    let body = match response.json::<GithubTrend>().await {
        Ok(body) => body,
        Err(_) => {
            return "json の parse に失敗しました。".to_string();
        }
    };

    for item in body.items {
        let name = item.full_name;
        let avatar_url = item.owner.avatar_url;
        let html_url = item.html_url;
        let description = item.description;
        let stars = item.stargazers_count;
        let _ = &command
            .channel_id
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

    return "".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("github_trend")
        .description("指定した言語のGitHub上でのトレンドを取得します")
        .create_option(|option| {
            option
                .name("language")
                .description("言語を入力してください")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
