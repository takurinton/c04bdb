use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let content = &msg.content;
    let content = content.replace("~random ", "");
    if content != "" {
        let items = content.split_whitespace().collect::<Vec<&str>>();
        let item = items[rand::random::<usize>() % items.len()];
        msg.channel_id.say(&ctx.http, item).await?;
    }
    Ok(())
}
