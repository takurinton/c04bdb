use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn praise(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "えらいねぇ").await?;
    Ok(())
}
