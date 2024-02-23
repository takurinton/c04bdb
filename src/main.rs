mod commands;
mod handler;
mod http;
mod url;
mod utils;

use std::env;

use serenity::framework::StandardFramework;
use serenity::prelude::*;

use crate::handler::Handler;

use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let token = match env::var("RINTON_DISCORD_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            error!("No token found in environment variable RINTON_DISCORD_TOKEN");
            return;
        }
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new();

    let mut client = match Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
    {
        Ok(client) => client,
        Err(why) => {
            error!("Error creating client: {:?}", why);
            return;
        }
    };

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    info!("Client started");
}
