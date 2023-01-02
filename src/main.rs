mod commands;

use std::env;

use regex::Regex;

use percent_encoding::percent_decode;

use serenity::framework::StandardFramework;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;

use crate::commands::ping::PING_COMMAND;
use crate::commands::cat::CAT_COMMAND;
use crate::commands::praise::PRAISE_COMMAND;
use crate::commands::friday::FRIDAY_COMMAND;
use crate::commands::image::IMAGE_COMMAND;

#[group]
#[commands(ping, cat, praise, friday, image)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // ping
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        // generate short link from healthy-person-emulator.memo.wiki/d/...
        // now this is not working...
        let content = msg.content.clone();
        let body = get_short_link(content).await;
        if body != "" {
            if let Err(why) = msg.channel_id.say(&ctx.http, body).await {
                println!("Error sending message: {:?}", why);
            }
        }

        // His saying "えらいねぇ" is not heartfelt at all...
        let message = msg.content.clone();
        let author = msg.author.name;
        let name = env::var("USER_Y").expect("user name is not defined");
        if author == name {
            if message == "えらいねぇ" || message == "すごいねぇ" {
                if let Err(why) = msg.channel_id.say(&ctx.http, format!("{author}の「{message}」は適当なんだよなぁ")).await {
                    println!("Error sending message: {:?}", why);
                }
            } else if message == "いいね" {
                if let Err(why) = msg.channel_id.say(&ctx.http, format!("本当にいいねって思ってんのか？")).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn get_short_link(content: String) -> String {
    let re = Regex::new(r"^(https://)?healthy-person-emulator\.memo\.wiki/d/.*").unwrap();
    if re.is_match(&content) {
        let url = match percent_decode(format!("https://is.gd/create.php?format=simple&url={content}").as_bytes()).decode_utf8() {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
        };

        let body = match reqwest::get(url).await {
            Ok(v) => match v.text().await {
                Ok(v) => v,
                Err(e) => panic!("Error: {}", e)
            },
            Err(e) => panic!("Error: {}", e)
        };

        body
    } else {
        String::from("")
    }
}

#[tokio::main]
#[test]
async fn test_get_short_link() {
    let content = String::from("https://healthy-person-emulator.memo.wiki/d/%c8%c8%ba%e1%a4%ce%bc%ab%cd%b3%a4%ac%a4%a2%a4%eb%a4%b3%a4%c8%a4%f2%c9%bd%cc%c0%a4%b9%a4%eb%a4%d9%a4%ad%a4%c7%a4%cf%a4%ca%a4%a4");
    let body = get_short_link(content).await;
    assert_eq!(body, "https://is.gd/2MTHvw");
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
    .configure(|c| c.prefix("~")).group(&GENERAL_GROUP);

    let mut client =
        Client::builder(&token, intents).framework(framework).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}