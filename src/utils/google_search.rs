use serde::Deserialize;
use std::env;
use tracing::error;

use crate::{
    http::client::{HttpClient, StatusCode},
    utils::encode::encode,
};

#[derive(Deserialize, Debug)]
pub struct GoogleItem {
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleResponse {
    pub items: Option<Vec<GoogleItem>>,
}

// google search api
// https://developers.google.com/custom-search/v1/using_rest
pub async fn google_search(
    q: &str,
    search_type: &str,
    site: &str,
) -> Result<Vec<GoogleItem>, String> {
    // web を明示するとエラーになるので省略する
    let search_type = if search_type == "image" {
        format!("&searchType={search_type}")
    } else {
        "".to_string()
    };
    let site = if site == "" {
        "".to_string()
    } else {
        format!("+site:{site}", site = site)
    };
    let search_engine_id = match env::var("SEARCH_ENGINE_ID") {
        Ok(id) => id,
        Err(_) => {
            error!("No token found in environment variable SEARCH_ENGINE_ID");
            return Err("SEARCH_ENGINE_ID が設定されていません。".to_string());
        }
    };
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("No token found in environment variable API_KEY");
            return Err("API_KEY が設定されていません。".to_string());
        }
    };

    let url = format!(
    "https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&hl=ja{search_type}&q={}{site}", encode(q));

    let client = HttpClient::new();
    let result = match client.get(&url).await {
        Ok(result) => match result.status_code {
            StatusCode::OK => result,
            StatusCode::BadRequest => return Err("リクエストが不正です。".to_string()),
            StatusCode::Unauthorized => return Err("認証に失敗しました。".to_string()),
            StatusCode::Forbidden => return Err("アクセス権限がありません。".to_string()),
            StatusCode::NotFound => return Err("リソースが見つかりませんでした。".to_string()),
            StatusCode::TooManyRequests => return Err(
                "Google Search API へのリクエスト超過です。しばらくしてからやり直してください。"
                    .to_string(),
            ),
            status => {
                return Err(
                    format!("予期しないエラーが発生しました。status: {}", status as u16)
                        .to_string(),
                );
            }
        },
        Err(_) => return Err("Google 検索でエラーが発生しました。".to_string()),
    };
    let body = match result.json::<GoogleResponse>().await {
        Ok(body) => body,
        Err(_) => return Err("jsonのparseに失敗しました。".to_string()),
    };
    let items = match body.items {
        Some(items) => items,
        None => return Err("検索結果がありません。".to_string()),
    };

    Ok(items)
}
