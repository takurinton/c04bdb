use std::env;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serenity::client::Context;

use crate::http::client::HttpClient;
use std::error::Error;

use super::get_db_channel::get_db_channel;

#[derive(Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CreateSessionResponse {
    accessJwt: String,
    // refreshJwt: String,
    // handle: String,
    // did: String,
    // didDoc: Option<String>,
    // email: Option<String>,
    // emailConfirmed: Option<bool>,
}

// TODO: 全体的にこのファイルは共通化する。今は feed とる以外しないから一旦ベタで書いていく。
// TODO: tracing でログを出すようにする。
async fn create_session() -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let mut client = HttpClient::new();
    let url = "https://bsky.social/xrpc/com.atproto.server.createSession";
    let identifier = match env::var("BSKY_IDENTIFIER") {
        Ok(identifier) => identifier,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "BSKY_IDENTIFIER が取得できません。",
            )));
        }
    };
    let password = match env::var("BSKY_PASS") {
        Ok(password) => password,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "BSKY_PASS が取得できません。",
            )));
        }
    };
    let body = format!(
        r#"{{"identifier":"{}","password":"{}"}}"#,
        identifier, password
    );

    let response = match client
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
        .post(url, body)
        .await
    {
        Ok(response) => response,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "API が取得できません。",
            )));
        }
    };

    let json = match response.json::<CreateSessionResponse>().await {
        Ok(json) => json,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "JSONのparseに失敗しました.",
            )));
        }
    };

    Ok(json)
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub feed: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    pub post: Post,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub uri: String,
    pub cid: String,
    pub author: Author,
    pub record: Record,
    // pub reply_count: u32,
    // pub repost_count: u32,
    // pub like_count: u32,
    // pub indexed_at: String,
    // pub viewer: Viewer,
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub did: String,
    pub handle: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub avatar: String,
    // pub viewer: Viewer,
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(rename = "$type")]
    pub record_type: String,
    #[warn(non_snake_case)]
    pub createdAt: String,
    pub langs: Vec<String>,
    pub text: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Viewer {
//     pub muted: bool,
//     pub blocked_by: bool,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {}

async fn get_feed() -> Result<Body, Box<dyn std::error::Error>> {
    let session = create_session().await?;
    let mut client = HttpClient::new();
    let params = "at://did:plc:c2f75sprlocrelfiftzblj6z/app.bsky.feed.generator/aaair5qf7emhe";
    let url = format!(
        "https://bsky.social/xrpc/app.bsky.feed.getFeed?feed={}",
        params
    );

    let response = match client
        .header_authorization(session.accessJwt)
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
        .get(&url)
        .await
    {
        Ok(response) => response,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "API が取得できません。",
            )));
        }
    };

    let json = match response.json::<Body>().await {
        Ok(json) => json,
        Err(why) => {
            println!("Error: {:?}", why);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "JSONのparseに失敗しました.",
            )));
        }
    };

    Ok(json)
}

pub async fn fetch_atproto(ctx: &Context) -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
    let res = get_feed().await?;
    let last_date = get_last_date(ctx).await?;
    let last_date = NaiveDateTime::parse_from_str(&last_date, "%Y-%m-%d %H:%M:%S")?;

    let feeds = res
        .feed
        .into_iter()
        .filter(|feed| {
            let created_at = NaiveDateTime::parse_from_str(
                &feed.post.record.createdAt,
                "%Y-%m-%dT%H:%M:%S%.3fZ",
            )
            .unwrap();
            created_at > last_date
        })
        .collect::<Vec<_>>();

    Ok(feeds)
}

async fn get_last_date(ctx: &Context) -> Result<String, Box<dyn Error>> {
    let db_channel = get_db_channel(ctx).await?;

    let messages = match db_channel
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await
    {
        Ok(messages) => messages
            .into_iter()
            .filter(|message| message.content.starts_with("atproto_last_date"))
            .collect::<Vec<_>>(),
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    // last_date は基本的に1つしか存在しないはずなので、0番目を取得
    let last_date = match messages.first() {
        // rss_last_date %Y-%m-%d %H:%M:%S という形式で保存されているので、それの日付部分だけを取得
        // nth(1) は %Y-%m-%d の部分, nth(2) は %H:%M:%S の部分
        Some(message) => {
            let date = message.content.split_whitespace().nth(1).unwrap();
            let time = message.content.split_whitespace().nth(2).unwrap();
            format!("{} {}", date, time)
        }
        None => chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
    Ok(last_date)
}
