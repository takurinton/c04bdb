use serde::Deserialize;

use super::google_search::google_search;
use crate::http::client::HttpClient;

#[derive(Deserialize)]
pub struct Pages {
    // pageid: i32,
    pub title: String,
    pub extract: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub pages: std::collections::HashMap<String, Pages>,
}

#[derive(Deserialize)]
pub struct Response {
    pub query: Query,
}

pub async fn wikipedia_search(search_text: &str) -> Result<(Response, String), String> {
    let items = match google_search(search_text, "web", "ja.wikipedia.org").await {
        Ok(items) => items,
        Err(err) => return Err(err),
    };

    let wikipedia = match items.iter().find(|item| match item.link.as_str() {
        link if link.starts_with("https://ja.wikipedia.org/wiki/") => true,
        _ => false,
    }) {
        Some(item) => item.link.clone(),
        None => return Err("wikipedia の検索に失敗しました。".to_string()),
    };

    let text = wikipedia.replace("https://ja.wikipedia.org/wiki/", "");
    let url = format!("https://ja.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={}", text);
    let client = HttpClient::new();
    let response = match client.get(&url).await {
        Ok(response) => response,
        Err(_) => {
            return Err("通信エラーが発生しました".to_string());
        }
    };

    let json = match response.json::<Response>().await {
        Ok(json) => json,
        Err(_) => {
            return Err("jsonのparseに失敗しました".to_string());
        }
    };

    Ok((json, text))
}
