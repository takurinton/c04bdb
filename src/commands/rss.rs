extern crate rand;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serenity::prelude::Context;

use crate::utils::get_db_channel::get_db_channel;

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> String {
    let operation = match options.iter().find(|option| option.name == "operation") {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => return "オペレーションが不正です".to_string(),
            },
            None => return "オペレーションが不正です".to_string(),
        },
        None => return "オペレーションが不正です".to_string(),
    };

    let link = match options.iter().find(|option| option.name == "link") {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(text) => text,
                _ => "",
            },
            None => "",
        },
        None => "",
    };

    let db_channel = match get_db_channel(ctx).await {
        Ok(db_channel) => db_channel,
        Err(why) => return why.to_string(),
    };

    let messages = match db_channel
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await
    {
        Ok(messages) => messages
            .into_iter()
            .filter(|message| message.content.starts_with("rss_link"))
            .collect::<Vec<_>>(),
        Err(_) => return "リンクの取得に失敗しました".to_string(),
    };

    match operation.as_str() {
        "add" => {
            // 重複チェック
            if messages
                .iter()
                .any(|x| x.content.split_whitespace().nth(1).unwrap() == link)
            {
                return format!("{} は既に登録されています。", link);
            }

            let _ = match db_channel
                .id
                .say(&ctx.http, format!("rss_link {}", link))
                .await
            {
                Ok(_) => "",
                Err(_) => return "リンクの登録に失敗しました。".to_string(),
            };

            return format!("{} を追加しました。", link);
        }
        "rm" => {
            match messages
                .iter()
                .position(|x| x.content.split_whitespace().nth(1).unwrap() == link)
            {
                Some(index) => {
                    let link_id = messages[index].id;
                    match db_channel.id.delete_message(&ctx.http, link_id).await {
                        Ok(_) => (),
                        Err(_) => return "リンクの削除に失敗しました".to_string(),
                    }
                    return format!("{} を削除しました。", link);
                }
                None => {
                    return format!("{} は見つかりませんでした。", link);
                }
            }
        }
        "ls" => {
            if messages.is_empty() {
                return "RSSが登録されていません。".to_string();
            } else {
                return format!(
                    "rss list は以下の通りです:
- {}",
                    messages
                        .iter()
                        .map(|x| format!("{}", x.content.split_whitespace().nth(1).unwrap()))
                        .collect::<Vec<_>>()
                        .join("\n- ")
                );
            }
        }
        _ => {}
    }

    "".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rss")
        .description("rss list を管理します")
        .create_option(|option| {
            option
                .name("operation")
                .kind(CommandOptionType::String)
                .description("オペレーション")
                .add_string_choice("add", "add")
                .add_string_choice("rm", "rm")
                .add_string_choice("ls", "ls")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("link")
                .kind(CommandOptionType::String)
                .description("リンク")
                .required(false)
        })
}
