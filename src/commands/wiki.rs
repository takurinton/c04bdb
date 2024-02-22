use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::utils::percent_decode::percent_decode;
use crate::utils::wikipedia_search::wikipedia_search;

pub async fn run(options: &[CommandDataOption]) -> String {
    let search_text = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "検索に失敗しました。".to_string(),
            },
            None => return "検索に失敗しました。".to_string(),
        },
        None => return "クエリが見つかりませんでした。".to_string(),
    };

    let (json, text) = match wikipedia_search(search_text).await {
        Ok(json) => json,
        Err(err) => return err,
    };

    let res = format!(
        "検索クエリ: {}
検索結果: {}
{}
https://ja.wikipedia.org/wiki/{}",
        search_text,
        percent_decode(text.as_str()),
        json.query.pages.iter().next().unwrap().1.extract,
        json.query.pages.iter().next().unwrap().1.title
    );

    res
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("wiki")
        .description("Wikipedia から検索して概要を返します。")
        .create_option(|option| {
            option
                .name("query")
                .description("検索する文字列")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
