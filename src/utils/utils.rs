use std::env;

use reqwest;
use reqwest::header;

use serde::Deserialize;

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

pub async fn fetch_chatgpt(content: String) -> String {
    let client = reqwest::Client::new();
    let response = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not defined"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(format!(
            r#"{{ "model": "gpt-3.5-turbo", "messages": [{{ "role": "user", "content": "{}" }}] }}"#,
            content
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return "通信エラーが発生しました。".to_string();
        }
    };

    let body = match response.json::<ChatGPTResponse>().await {
        Ok(body) => body,
        Err(_) => {
            return "コンテンツの取得に失敗しました。".to_string();
        }
    };

    body.choices[0].message.content.clone()
}

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

    let result = match reqwest::get(&url).await {
        Ok(result) => {
            if result.status() == reqwest::StatusCode::OK {
                result
            } else {
                return Err(format!(
                    "Google 検索でエラーが発生しました。ステータスコード: {}",
                    result.status()
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

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "Rinton")
        .send();

    let response = match response.await {
        Ok(response) => response,
        Err(_) => return Err("Github でエラーが発生しました。".to_string()),
    };

    let _ = match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => return Err("認証に失敗しました。".to_string()),
        reqwest::StatusCode::FORBIDDEN => return Err("アクセス権限がありません。".to_string()),
        reqwest::StatusCode::NOT_FOUND => {
            return Err("リソースが見つかりませんでした。".to_string())
        }
        _ => return Err("予期しないエラーが発生しました。".to_string()),
    };

    let body = match response.json::<GithubTrend>().await {
        Ok(body) => body,
        Err(_) => return Err("jsonのparseに失敗しました。".to_string()),
    };

    Ok(body.items)
}

fn hex_to_u8(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("Invalid hex digit: {}", hex),
    }
}

pub fn percent_decode(input: &str) -> String {
    use std::str;

    let mut out = Vec::new();
    let mut i = 0;
    let bytes = input.as_bytes();
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let h = hex_to_u8(bytes[i + 1]);
                let l = hex_to_u8(bytes[i + 2]);
                out.push((h << 4) | l);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    str::from_utf8(&out).unwrap().to_string()
}
