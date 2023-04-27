use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn run(options: &[CommandDataOption]) -> String {
    let random_target = match options.get(0) {
        Some(option) => match &option.resolved {
            Some(value) => match value {
                CommandDataOptionValue::String(random) => random,
                _ => return "選択対象を入力してください".to_string(),
            },
            None => return "選択対象を入力してください".to_string(),
        },
        None => return "選択対象を入力してください".to_string(),
    };

    let items = random_target.split_whitespace().collect::<Vec<&str>>();
    let item = items[rand::random::<usize>() % items.len()];

    format!(
        "選択対象:{}
選択結果:{}",
        random_target, item,
    )
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("random")
        .description("ランダムに選択する")
        .create_option(|option| {
            option
                .name("random")
                .description("検索する文字列をスペース区切りで入力")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
