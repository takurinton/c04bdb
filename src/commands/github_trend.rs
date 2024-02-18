use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::prelude::*;

use crate::utils::github_search::github_search;

pub async fn run(command: &ApplicationCommandInteraction, ctx: &Context) -> String {
    let options = &command.data.options;
    let language = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "言語を指定してください".to_string(),
            },
            None => return "言語を指定してください".to_string(),
        },
        None => return "言語を指定してください".to_string(),
    };

    let items = match github_search(language).await {
        Ok(items) => items,
        Err(err) => return err,
    };
    for item in items {
        let name = item.full_name;
        let avatar_url = item.owner.avatar_url;
        let html_url = item.html_url;
        let description = item.description;
        let stars = item.stargazers_count;
        let _ = &command
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|a| {
                        a.name(name).url(html_url).icon_url(avatar_url);
                        a
                    });
                    e.description(description);
                    e.field("Stars", stars, true);
                    e
                });
                m
            })
            .await;
    }

    return "".to_string();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("github_trend")
        .description("指定した言語のGitHub上でのトレンドを取得します")
        .create_option(|option| {
            option
                .name("language")
                .description("言語を入力してください")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
