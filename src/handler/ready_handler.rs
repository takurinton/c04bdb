use crate::commands;
use crate::utils::fetch_rss_feed::fetch_rss_feed;
use crate::utils::get_db_channel::get_db_channel;
use serenity::{
    client::Context,
    model::id::{ChannelId, GuildId},
    utils::colours,
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
        let _ = channel
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title(match &feed.title {
                        Some(title) => title,
                        None => "タイトルなし",
                    });
                    e.url(match &feed.link {
                        Some(link) => link,
                        None => "",
                    });
                    e.description(match &feed.description {
                        Some(description) => description,
                        None => "",
                    });
                    // e.timestamp(match &feed.pub_date {
                    //     Some(pub_date) => {
                    //         println!("{:?}", pub_date);
                    //         let pub_date = match NaiveDateTime::parse_from_str(pub_date, "%a, %d %b %Y %H:%M:%S %Z") {
                    //             Ok(date) => date,
                    //             Err(why) => {
                    //                 println!("Error parsing date: {:?}", why);
                    //                 chrono::Utc::now().naive_utc()
                    //             }
                    //         };
                    //         pub_date.to_string()
                    //     },
                    //     None => {
                    //         let now = chrono::Utc::now();
                    //         now.naive_utc().to_string()
                    //     }
                    // });
                    e.color(colours::branding::BLURPLE);
                    e
                });
                m
            })
            .await;
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
