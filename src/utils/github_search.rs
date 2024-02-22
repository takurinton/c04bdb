use serde::Deserialize;

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
        Err(_) => return Err("Github でエラーが発生しました。".to_string()),
    };

    let _ = match response.status_code {
        StatusCode::OK => (),
        StatusCode::Unauthorized => return Err("認証に失敗しました。".to_string()),
        StatusCode::Forbidden => return Err("アクセス権限がありません。".to_string()),
        StatusCode::NotFound => return Err("リソースが見つかりませんでした。".to_string()),
        _ => return Err("予期しないエラーが発生しました。".to_string()),
    };

    let body = match response.json::<GithubTrend>().await {
        Ok(body) => body,
        Err(_) => return Err("jsonのparseに失敗しました。".to_string()),
    };

    Ok(body.items)
}
