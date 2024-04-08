use std::error::Error;

use rss::Item;
use serenity::{async_trait, client::Context, model::id::ChannelId};
use tracing::{error, info, warn};

use crate::utils::get_db_channel::get_db_channel;

use super::processer::Processer;

use crate::utils::fetch_atproto::fetch_atproto;

pub(crate) struct ProcesserStruct;

#[async_trait]
impl Processer for ProcesserStruct {
    async fn fetch(&self, ctx: &Context) -> Result<Vec<Item>, Box<dyn Error>> {
        let res = fetch_atproto().await;
        println!("{:?}", res);
        Ok(vec![])
    }

    async fn post_to_channel(&self, ctx: &Context, items: Vec<Item>) -> Result<(), Box<dyn Error>> {
        let channel = ChannelId(1208611584964825099);
        for item in items {
            let _ = match item.link {
                Some(link) => {
                    let _ = channel.send_message(&ctx.http, |m| m.content(link)).await;
                }
                None => {
                    warn!("No link found in atproto feed: {:?}", item.title);
                }
            };
        }
        Ok(())
    }

    async fn update_db_channel(&self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let db_channel = match get_db_channel(&ctx).await {
            Ok(db_channel) => db_channel,
            Err(why) => {
                error!("Error getting db channel: {:?}", why);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "DBチャンネルが見つかりません。",
                )));
            }
        };

        let messages = db_channel
            .messages(&ctx.http, |retriever| retriever.limit(1))
            .await
            .unwrap()
            .into_iter()
            .filter(|message| message.content.starts_with("atproto_last_date"))
            .collect::<Vec<_>>();

        for message in messages {
            let _ = message.delete(&ctx.http).await;
        }

        let now = chrono::Utc::now();
        let now = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let _ = db_channel
            .send_message(&ctx.http, |m| {
                m.content(format!("atproto_last_date {}", now))
            })
            .await;
        Ok(())
    }

    async fn run(&self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        info!("atproto retrieval is started.");

        let items = self.fetch(ctx).await?;
        // self.post_to_channel(ctx, items).await?;
        // self.update_db_channel(ctx).await?;

        info!("atproto retrieval is done.");

        Ok(())
    }
}
