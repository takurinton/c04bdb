use serde::Deserialize;
use std::env;

use crate::http::client::HttpClient;

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
    let search_engine_id = env::var("SEARCH_ENGINE_ID").expect("search engine id is not defined");
    let api_key = env::var("API_KEY").expect("api key is not defined");
    let url = format!(
    "https://www.googleapis.com/customsearch/v1?cx={search_engine_id}&key={api_key}&hl=ja{search_type}&q={q}{site}");

    let client = HttpClient::new();
    let result = match client.get(&url).await {
        Ok(result) => {
            if result.status_code == 200 {
                result
            } else {
                return Err(format!(
                    "Google 検索でエラーが発生しました。ステータスコード: {}",
                    result.status_code
                ));
            }
        }
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
