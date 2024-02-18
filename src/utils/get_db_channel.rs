use serenity::{
    client::Context,
    model::{
        channel::GuildChannel,
        id::{ChannelId, GuildId},
    },
};
use std::env;
use std::error::Error;

// db チャンネルを取得
pub async fn get_db_channel(ctx: &Context) -> Result<GuildChannel, Box<dyn Error>> {
    let guild_id = GuildId(889012300705591307);

    let db_channel_id = match env::var("DISCORD_DB_CHANNEL_ID_RINTON_BOT")
        .expect("search engine id is not defined")
        .parse::<u64>()
    {
        Ok(db_channel_id) => db_channel_id,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    let channels = match guild_id.channels(&ctx.http).await {
        Ok(channel) => channel,
        Err(_) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    let db_channel = match channels.get(&ChannelId(db_channel_id)) {
        Some(channel) => channel,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "DBチャンネルが見つかりません。",
            )))
        }
    };

    Ok(db_channel.clone())
}
