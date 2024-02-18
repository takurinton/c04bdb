use serenity::prelude::*;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

mod interaction_create_handler;
mod message_handler;
mod ready_handler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        message_handler::message(ctx, message).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create_handler::interaction_create(ctx, interaction).await;
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        ready_handler::ready(ctx).await;
    }
}
