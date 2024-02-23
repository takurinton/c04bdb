use serde::Deserialize;
use tracing::{error, warn};

use crate::http::client::{HttpClient, StatusCode};

#[derive(Deserialize, Debug)]
pub struct GoogleItem {
    pub link: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleResponse {
    pub items: Option<Vec<GoogleItem>>,
}

#[derive(Deserialize)]
pub struct Owner {
    pub avatar_url: String,
}

#[derive(Deserialize)]
pub struct GithubTrendItem {
    pub full_name: String,
    pub html_url: String,
    pub description: String,
    pub stargazers_count: u32,
    pub owner: Owner,
}

#[derive(Deserialize)]
struct GithubTrend {
    pub items: Vec<GithubTrendItem>,
}

// github api
// https://docs.github.com/en/rest/reference/search#search-code
pub async fn github_search(language: &str) -> Result<Vec<GithubTrendItem>, String> {
    let url = format!("https://api.github.com/search/repositories?q=language:{language}&order=desc&per_page=10&since=daily");

    let client = HttpClient::new();
    let response = client.get(&url);

    let response = match response.await {
        Ok(response) => response,
        Err(_) => {
            warn!("github api request failed");
            return Err("ネットワークエラーです。".to_string());
        }
    };

    let _ = match response.status_code {
        StatusCode::OK => (),
        StatusCode::NotFound => {
            error!("github api not found");
            return Err("リソースが見つかりませんでした。".to_string());
        }
        _ => {
            error!("github api error");
            return Err("エラーが発生しました。".to_string());
        }
    };

    let body = match response.json::<GithubTrend>().await {
        Ok(body) => body,
        Err(_) => {
            error!("json parse failed");
            return Err("json の parse に失敗しました。".to_string());
        }
    };

    Ok(body.items)
}
