use std::env;

use rand::Rng;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = &msg.content;
        // guild_id はどこから参照しても同じ値なので最初に取得しておく
        let guild_id = match &msg.guild_id {
            Some(id) => id.0,
            None => return,
        };

        // health check
        let mentions = msg.mentions;
        if mentions.len() > 0 {
            let bot_id = match env::var("DISCORD_BOT_ID") {
                Ok(id) => id,
                Err(_) => return,
            };

            for mention in mentions {
                if mention.id.0.to_string() == bot_id {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "なんや").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }

        // discord message url
        let re = match regex::Regex::new(r"https://discord.com/channels/\d+/\d+/\d+") {
            Ok(re) => re,
            Err(_) => return,
        };
        // if message is discord message url, send opened message
        if re.is_match(content) {
            // ids from url: e.g. https://discord.com/channels/{guild_id}/{channel_id}/{message_id}
            let channel_id = match content.split("/").nth(5) {
                Some(id) => match id.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => return,
                },
                None => return,
            };
            let message_id = match content.split("/").nth(6) {
                Some(id) => match id.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => return,
                },
                None => return,
            };

            if let Some(message) = ChannelId(channel_id)
                .message(&ctx.http, message_id)
                .await
                .ok()
            {
                // message with break
                let message_with_break = message.content.replace("\n", "\n> ");
                // image
                let attachments = message
                    .attachments
                    .iter()
                    .map(|a| a.url.clone())
                    .collect::<Vec<String>>()
                    .join(" ");

                let message_with_attachments = format!("{} {}", message_with_break, attachments);

                // guild info
                let guild = GuildId(guild_id);

                // user emoji
                let emojis = match guild.emojis(&ctx.http).await {
                    Ok(emojis) => emojis,
                    Err(_) => return,
                };
                let emoji = match emojis.iter().find(|e| e.name == message.author.name) {
                    Some(emoji) => emoji,
                    // default emoji
                    // unwrap is safe because neko emoji is always exists
                    None => emojis.iter().find(|e| e.name == "neko").unwrap(),
                };

                // user display name
                let display_name = match message.author.nick_in(&ctx.http, guild).await {
                    Some(nick) => nick,
                    None => message.author.name.clone(),
                };
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "{} **{}** in <#{}>
> {}",
                            emoji, display_name, channel_id, message_with_attachments
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }

        // His saying "えらいねぇ" is not heartfelt at all...
        let author = msg.author.name;
        let name = env::var("USER_Y").expect("user name is not defined");
        if author == name {
            if content == "えらいねぇ" || content == "すごいねぇ" {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("{author}の「{content}」は適当なんだよなぁ"),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            } else if content == "いいね" {
                let res_list = ["本当にいいねって思ってんのか？", "いいね(笑)"];
                let index = rand::thread_rng().gen_range(0..res_list.len());
                if let Err(why) = msg.channel_id.say(&ctx.http, res_list[index]).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
