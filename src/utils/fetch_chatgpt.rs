use serde::Deserialize;
use std::env;
use tracing::error;

use crate::http::client::HttpClient;

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

pub async fn fetch_chatgpt(content: String, prompts: Vec<String>) -> String {
    let prompts = prompts
        .iter()
        .map(|p| format!(r#"{{ "role": "system", "content": "{}" }},"#, p))
        .collect::<Vec<String>>()
        .join("");

    let request_body = format!(
        r#"{{ "model": "gpt-3.5-turbo", "messages": [{}{{ "role": "user", "content": "{}" }}] }}"#,
        prompts, content
    );

    let open_api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("OPENAI_API_KEY is not defined");
            return "APIキーが設定されていません。".to_string();
        }
    };

    let mut client = HttpClient::new();
    let response = match client
        .set_header("Content-Type", "application/json")
        .header_authorization(open_api_key)
        .post("https://api.openai.com/v1/chat/completions", request_body)
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("failed to get chatgpt: {:?}", e);
            return "通信エラーが発生しました。".to_string();
        }
    };

    let body = match response.json::<ChatGPTResponse>().await {
        Ok(body) => body,
        Err(e) => {
            error!("failed to parse json: {:?}", e);
            return "コンテンツの取得に失敗しました。".to_string();
        }
    };

    body.choices[0].message.content.clone()
}
