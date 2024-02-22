use chrono::{DateTime, NaiveDateTime};
use rss::{Channel, Item};
use serenity::client::Context;
use std::error::Error;

use super::get_db_channel::get_db_channel;
use crate::http::client::HttpClient;

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
    let client = HttpClient::new();
    let result = match client.get(&url).await {
        Ok(content) => content,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "URL が見つかりません。",
            )))
        }
    };
    let content = result.body.as_bytes();
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
    // ラグ対策として半日巻き戻す
    let last_date = last_date - chrono::Duration::hours(12);

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
            let date = match DateTime::parse_from_rfc2822(date) {
                Ok(date) => date - chrono::Duration::hours(12),
                Err(_) => {
                    // 形式が正しくなかったら除外するために 1970年1月1日を GMT 形式で DateTime に parse して返す
                    DateTime::parse_from_rfc2822("Thu, 01 Jan 1970 00:00:00 GMT")?
                }
            };

            if date.naive_utc() > last_date {
                items.push(item.clone());
            }
        }
    }

    Ok(items)
}
