use std::env;

use serde::{Deserialize, Serialize};

use crate::http::client::HttpClient;

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

pub async fn fetch_atproto() -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
    let res = get_feed().await?;
    Ok(res.feed)
}
