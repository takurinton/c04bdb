extern crate rand;

use rand::Rng;

fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    // 1 〜 21
    let n = rng.gen_range(1..22);
    n
}

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    let n = get_random_number();
    let image_url = format!("https://takurinton.dev/hash{n}.jpeg");

    image_url
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cat")
        .description("ハッシュの画像をランダムで返します")
}
