use std::env;

use chrono::NaiveDateTime;
use reqwest;
use reqwest::header;

use serde::Deserialize;
use serenity::{
    client::Context,
    model::{
        channel::GuildChannel,
        id::{ChannelId, GuildId},
    },
};

use rss::{Channel, Item};
use std::error::Error;

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
    let client = reqwest::Client::new();
    let prompts = prompts
        .iter()
        .map(|p| format!(r#"{{ "role": "system", "content": "{}" }},"#, p))
        .collect::<Vec<String>>()
        .join("");
    let response = match client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not defined"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(format!(
            r#"{{ "model": "gpt-3.5-turbo", "messages": [{}{{ "role": "user", "content": "{}" }}] }}"#,
            prompts,
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

// rss のリストを #db チャンネルから `rss_link` という prefix がついてるものを取得。
// TODO: コマンド経由で追加できるようにする
async fn get_rss_list(ctx: &Context) -> Result<Vec<String>, Box<dyn Error>> {
    let db_channel = get_db_channel(ctx).await?;

    let messages = match db_channel
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await
    {
        Ok(messages) => messages
            .into_iter()
            .filter(|message| message.content.starts_with("rss_link"))
            .collect::<Vec<_>>(),
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    let rss_list = messages
        .iter()
        .map(|message| {
            message
                .content
                .split_whitespace()
                .nth(1)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();

    Ok(rss_list)
}

async fn fetch_feed(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn get_last_date(ctx: &Context) -> Result<String, Box<dyn Error>> {
    let db_channel = get_db_channel(ctx).await?;

    let messages = match db_channel
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await
    {
        Ok(messages) => messages
            .into_iter()
            .filter(|message| message.content.starts_with("rss_last_date"))
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

pub async fn fetch_rss_feed(ctx: &Context) -> Result<Vec<Item>, Box<dyn Error>> {
    let rss_list = get_rss_list(ctx).await?;

    let last_date = get_last_date(ctx).await?;
    let last_date = NaiveDateTime::parse_from_str(&last_date, "%Y-%m-%d %H:%M:%S")?;

    let mut items = Vec::new();
    for url in rss_list {
        let channel = match fetch_feed(url).await {
            Ok(channel) => channel,
            Err(_) => continue,
        };

        for item in channel.items() {
            // RSS の pubDate の形式の仕様は RFC 822 に準拠している
            // https://www.rssboard.org/rss-draft-1#data-types-datetime
            // https://validator.w3.org/feed/docs/error/InvalidRFC2822Date.html
            let date = match item.pub_date.as_ref() {
                Some(date) => date,
                None => "",
            };
            let date = match NaiveDateTime::parse_from_str(&date, "%a, %d %b %Y %H:%M:%S %Z") {
                Ok(date) => date,
                Err(_) => {
                    // 形式が正しくなかったら除外するために 1970年1月1日を "%a, %d %b %Y %H:%M:%S %Z" の形式で parse する
                    NaiveDateTime::parse_from_str(
                        "Sun, 01 Jan 1970 00:00:00 GMT",
                        "%a, %d %b %Y %H:%M:%S %Z",
                    )?
                }
            };
            if date > last_date {
                items.push(item.clone());
            }
        }
    }

    Ok(items)
}

// db チャンネルを取得
pub async fn get_db_channel(ctx: &Context) -> Result<GuildChannel, Box<dyn Error>> {
    let guild_id = GuildId(889012300705591307);

    let db_channel_id = match env::var("DISCORD_DB_CHANNEL_ID_RINTON_BOT")
        .expect("search engine id is not defined")
        .parse::<u64>()
    {
        Ok(db_channel_id) => db_channel_id,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    let channels = match guild_id.channels(&ctx.http).await {
        Ok(channel) => channel,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    let db_channel = match channels.get(&ChannelId(db_channel_id)) {
        Some(channel) => channel,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    Ok(db_channel.clone())
}
