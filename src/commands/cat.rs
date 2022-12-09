use std::env;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let n = rand::random::<u32>() % 10 + 1;
    let NEKO_URL = env::var("NEKO_URL").expect("NEKO_URL is not defined");
    let image_url = format!("{NEKO_URL}{n}.jpeg");
    msg.channel_id.say(&ctx.http, image_url).await?;
    Ok(())
}