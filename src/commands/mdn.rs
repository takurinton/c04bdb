use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::commands::utils::google_search;

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

    let items = match google_search(text, "", "developer.mozilla.org").await {
        Ok(items) => items,
        Err(err) => return err,
    };

    let url = match items
        .iter()
        .find(|item| item.link.as_str().contains("developer.mozilla.org/ja"))
    {
        Some(item) => item.link.as_str().to_string(),
        None => match items.get(0) {
            Some(item) => item.link.as_str().to_string(),
            None => {
                return "検索結果が見つかりませんでした。".to_string();
            }
        },
    };

    format!(
        "
検索文字列: {text}
{url}",
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("mdn")
        .description("developer.mozilla.org から検索して概要を返します。")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
