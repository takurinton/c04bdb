use std::env;

use serde::Deserialize;

use crate::http::client::HttpClient;

#[derive(Deserialize)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct CreateSessionResponse {
    accessJwt: String,
    refreshJwt: String,
    handle: String,
    did: String,
    didDoc: Option<String>,
    email: Option<String>,
    emailConfirmed: Option<bool>,
}

// TODO: 全体的にこのファイルは共通化する。今は feed とる以外しないから一旦ベタで書いていく。
// TODO: tracing でログを出すようにする。
async fn create_session() -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let client = HttpClient::new();
    let url = "https://bsky.social/xrpc/com.atproto.server.createSession";
    let identifier = env::var("BSKY_IDENTIFIER")?;
    let password = env::var("BSKY_PASSWORD")?;
    let body = format!(
        r#"{{"identifier":"{}","password":"{}"}}"#,
        identifier, password
    );

    let response = match client.post(url, body).await {
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
                "JSON が取得できません。",
            )));
        }
    };

    Ok(json)
}

pub async fn fetch_atproto() -> Result<CreateSessionResponse, Box<dyn std::error::Error>> {
    let session = create_session().await?;
    Ok(session)
}
