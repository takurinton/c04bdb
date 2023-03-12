extern crate rand;

use std::env;

use rand::Rng;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    // 1 ~ 21
    let n = rng.gen_range(1..22);
    n
}

#[command]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let n = get_random_number();
    let neko_url = env::var("NEKO_URL").expect("NEKO_URL is not defined");
    let image_url = format!("{neko_url}{n}.jpeg");
    msg.channel_id.say(&ctx.http, image_url).await?;
    Ok(())
}
