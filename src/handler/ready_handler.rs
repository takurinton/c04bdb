use std::env;

use crate::{commands, scheduler::processer::Processer};

use serenity::{client::Context, model::id::GuildId};
use tracing::{error, info};

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

    let mode = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    // 30分に1回、プロダクションのときのみ実行
    if mode == "production" {
        // RSS feed を取得する
        let rss_processor = crate::scheduler::rss::ProcesserStruct;
        match rss_processor.run(&ctx).await {
            Ok(_) => info!("RSS feed fetched successfully."),
            Err(why) => error!("Error fetching RSS feed: {:?}", why),
        }

        // 部分ツイートを取得する
        let atproto_processor = crate::scheduler::atproto::ProcesserStruct;
        match atproto_processor.run(&ctx).await {
            Ok(_) => info!("部分ツイート fetched successfully."),
            Err(why) => error!("Error fetching 部分ツイート: {:?}", why),
        }
    } else {
        info!("RSS and 部分ツイート are not performed in development mode.");
    }

    info!("bot is ready!")
}
