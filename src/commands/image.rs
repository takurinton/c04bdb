use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::utils::utils::google_search;

pub async fn run(options: &[CommandDataOption]) -> String {
    let text = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "検索に失敗しました。".to_string(),
            },
            None => return "検索に失敗しました。".to_string(),
        },
        None => return "クエリが見つかりませんでした。".to_string(),
    };

    let items = match google_search(text, "image", "").await {
        Ok(items) => items,
        Err(err) => return err,
    };
    let length = items.len();
    let random = rand::random::<usize>() % length;
    let item = &items[random];

    format!(
        "検索クエリ:{}
{}",
        text,
        item.link.clone()
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("image")
        .description("Google画像検索の結果をランダムに返します")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
