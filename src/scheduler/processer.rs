use rss::Item;
use serenity::async_trait;
use serenity::client::Context;
use std::error::Error;

#[async_trait]
pub trait Processer {
    async fn fetch(&self, ctx: &Context) -> Result<Vec<Item>, Box<dyn Error>>;
    async fn post_to_channel(&self, ctx: &Context, items: Vec<Item>) -> Result<(), Box<dyn Error>>;
    async fn update_db_channel(&self, ctx: &Context) -> Result<(), Box<dyn Error>>;
    async fn run(&self, ctx: &Context) -> Result<(), Box<dyn Error>>;
}
