use crate::commands;
use crate::utils::fetch_rss_feed::fetch_rss_feed;
use crate::utils::get_db_channel::get_db_channel;
use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
};

pub async fn ready(ctx: Context) {
    let guild_id = GuildId(889012300705591307);

    let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands.create_application_command(|command| commands::random::register(command));
        commands.create_application_command(|command| commands::friday::register(command));
        commands.create_application_command(|command| commands::cat::register(command));
        commands.create_application_command(|command| commands::wiki::register(command));
        commands.create_application_command(|command| commands::eval::register(command));
        commands.create_application_command(|command| commands::todo::register(command));
        commands.create_application_command(|command| commands::image::register(command));
        commands.create_application_command(|command| commands::github_trend::register(command));
        commands.create_application_command(|command| commands::mdn::register(command));
        commands.create_application_command(|command| commands::levenshtein::register(command));
        commands.create_application_command(|command| commands::line::register(command));
        commands.create_application_command(|command| commands::rss::register(command))
    })
    .await;

    let feeds = match fetch_rss_feed(&ctx).await {
        Ok(feeds) => feeds,
        Err(why) => {
            println!("Error fetching rss feed: {:?}", why);
            vec![]
        }
    };

    // channel に post
    let channel = ChannelId(1208611584964825099);
    for feed in feeds {
        let _ = match feed.link {
            // link が存在したらそのまま送信する
            Some(link) => {
                let _ = channel.send_message(&ctx.http, |m| m.content(link)).await;
            }
            // 何もなかったら何もしない
            None => {
                println!("link がありません");
            }
        };
    }

    let db_channel = get_db_channel(&ctx).await.unwrap();

    // rss_last_date prefix がついているメッセージを削除
    let messages = db_channel
        .messages(&ctx.http, |retriever| retriever.limit(1))
        .await
        .unwrap()
        .into_iter()
        .filter(|message| message.content.starts_with("rss_last_date"))
        .collect::<Vec<_>>();

    for message in messages {
        let _ = message.delete(&ctx.http).await;
    }

    // 日付を更新
    let now = chrono::Utc::now();
    let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let _ = db_channel
        .send_message(&ctx.http, |m| m.content(format!("rss_last_date {}", now)))
        .await;

    println!("connected!");
}
