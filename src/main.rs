mod commands;
mod message;

use std::env;

use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::prelude::*;

use crate::commands::cat::CAT_COMMAND;
use crate::commands::chatgpt::CHATGPT_COMMAND;
use crate::commands::eval::EVAL_COMMAND;
use crate::commands::friday::FRIDAY_COMMAND;
use crate::commands::image::IMAGE_COMMAND;
use crate::commands::praise::PRAISE_COMMAND;
use crate::commands::random::RANDOM_COMMAND;

use crate::message::message::Handler;

#[group]
#[commands(cat, praise, friday, image, random, chatgpt, eval)]
struct General;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
