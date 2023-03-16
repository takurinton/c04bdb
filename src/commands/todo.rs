
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
async fn todo(ctx: &Context, msg: &Message) -> CommandResult {
    let operation_list = ["add", "rm", "ls"];
    let operation = match msg.content.split_whitespace().nth(1) {
        Some(operation) => match operation_list.contains(&operation) {
            true => operation,
            false => {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "不正な操作です。{} のどれかを指定してください",
                            operation_list.join(", ")
                        ),
                    )
                    .await?;
                return Ok(());
            }
        },
        None => {
            msg.channel_id
                .say(&ctx.http, "操作を指定してください。")
                .await?;
            return Ok(());
        }
    };

    let db_channel_id = env::var("DISCORD_DB_CHANNEL_ID")
        .expect("search engine id is not defined")
        .parse::<u64>()?;
    let guild_id = match msg.guild_id {
        Some(guild_id) => guild_id,
        None => return Ok(()),
    };
    let channels = match guild_id.channels(&ctx.http).await {
        Ok(channel) => channel,
        Err(_) => return Ok(()),
    };
    let db_channel = match channels.get(&ChannelId(db_channel_id)) {
        Some(channel) => channel,
        None => return Ok(()),
    };

    let messages = match db_channel
        .messages(&ctx.http, |retriever| retriever.limit(100))
        .await
    {
        Ok(messages) => messages
            .into_iter()
            .filter(|message| message.content.starts_with("todo_message"))
            .collect::<Vec<_>>(),
        Err(_) => return Ok(()),
    };

    match operation {
        "add" => {
            let todo_message = match msg.content.split_whitespace().nth(2) {
                Some(todo_message) => todo_message,
                None => {
                    msg.channel_id
                        .say(&ctx.http, "追加するメッセージを指定してください。")
                        .await?;
                    return Ok(());
                }
            };
            let id = match messages.len() {
                0 => 1,
                _ => {
                    let mut ids = messages
                        .iter()
                        .map(|message| {
                            message
                                .content
                                .split_whitespace()
                                .nth(1)
                                .unwrap()
                                .parse::<u64>()
                                .unwrap()
                        })
                        .collect::<Vec<_>>();
                    ids.sort();
                    ids.last().unwrap() + 1
                }
            };
            db_channel
                .say(&ctx.http, format!("todo_message {} {}", id, todo_message))
                .await?;
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("{}: {} を追加しました。", id, todo_message),
                )
                .await?;
        }
        "rm" => {
            let todo_message = match msg.content.split_whitespace().nth(2) {
                Some(todo_message) => todo_message,
                None => {
                    msg.channel_id
                        .say(&ctx.http, "削除するメッセージを指定してください。")
                        .await?;
                    return Ok(());
                }
            };
            match messages.iter().position(|x| {
                x.content.split_whitespace().nth(1).unwrap() == todo_message
                    || x.content.split_whitespace().nth(2).unwrap() == todo_message
            }) {
                Some(index) => {
                    let todo_message_id = messages[index].id;
                    ChannelId(db_channel_id)
                        .delete_message(&ctx.http, todo_message_id)
                        .await?;
                    msg.channel_id
                        .say(&ctx.http, format!("{} を削除しました。", todo_message))
                        .await?;
                }
                None => {
                    msg.channel_id
                        .say(&ctx.http, format!("{} は存在しません。", todo_message))
                        .await?;
                }
            }
        }
        "ls" => {
            if messages.is_empty() {
                msg.channel_id
                    .say(&ctx.http, "TODOリストには何もありません。")
                    .await?;
            } else {
                msg.channel_id
                    .say(
                        &ctx.http,
                        format!(
                            "TODOリスト:
・{}",
                            messages
                                .iter()
                                .map(|x| format!(
                                    "{} {}",
                                    x.content.split_whitespace().nth(1).unwrap(),
                                    x.content.split_whitespace().nth(2).unwrap()
                                ))
                                .collect::<Vec<_>>()
                                .join("\n・")
                        ),
                    )
                    .await?;
            }
        }
        _ => {}
    }

    Ok(())
}
