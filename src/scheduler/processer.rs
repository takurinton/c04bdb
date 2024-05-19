use serenity::async_trait;
use serenity::client::Context;
use std::error::Error;

#[async_trait]
pub trait Processer<T> {
    async fn fetch(&self, ctx: &Context) -> Result<Vec<T>, Box<dyn Error>>;
    async fn post_to_channel(&self, ctx: &Context, items: Vec<T>) -> Result<(), Box<dyn Error>>;
    async fn update_db_channel(&self, ctx: &Context) -> Result<(), Box<dyn Error>>;
    async fn run(&self, ctx: &Context) -> Result<(), Box<dyn Error>>;
}
