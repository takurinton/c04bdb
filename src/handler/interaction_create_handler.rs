use serenity::{
    client::Context,
    model::application::interaction::{Interaction, InteractionResponseType},
};
use tracing::{error, info};

use crate::commands;

pub async fn interaction_create(ctx: Context, interaction: Interaction) {
    if let Interaction::ApplicationCommand(command) = interaction {
        info!("called command: {:?}", command.data.name);
        let content = match command.data.name.as_str() {
            "random" => commands::random::run(&command.data.options),
            "friday" => commands::friday::run(&command.data.options),
            "cat" => commands::cat::run(&command.data.options),
            "wiki" => commands::wiki::run(&command.data.options).await,
            "eval" => commands::eval::run(&command.data.options).await,
            "todo" => commands::todo::run(&command.data.options, &ctx).await,
            "image" => commands::image::run(&command.data.options).await,
            "github_trend" => commands::github_trend::run(&command, &ctx).await,
            "mdn" => commands::mdn::run(&command.data.options).await,
            "levenshtein" => commands::levenshtein::run(&command.data.options),
            "line" => commands::line::run(&command.data.options),
            "rss" => commands::rss::run(&command.data.options, &ctx).await,
            _ => "not implemented :(".to_string(),
        };

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
        {
            error!("failed to create interaction response: {:?}", why);
        }
    }
}
