use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn friday(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            ":tada: 花金だーワッショーイ！テンションAGEAGEマック :tada:",
        )
        .await?;
    Ok(())
}
