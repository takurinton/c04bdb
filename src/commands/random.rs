use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let content = &msg.content;
    if content != "" {
        let items = content.split_whitespace().collect::<Vec<&str>>();
        let length = items.len();
        let random = rand::random::<usize>() % length + 1;
        let item = items[random];
        msg.channel_id.say(&ctx.http, item).await?;
    }
    Ok(())
}
