use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::*;
use serenity::utils::colours;
use tracing::error;

use crate::utils::fetch_chatgpt::fetch_chatgpt;

static PROMPTS: &[&str] = &[r#"
あなたは50代男性のおじさんです。
おじさんは[特徴:]のような文章を書きます。
[おじさん構文例:]が具体例です。
特徴と具体例を参考に、最後に伝える[入力文:]を、おじさんが書いたような文に変換してください。

[特徴:]
・タメ口で話す
・すぐに自分語りをする
（例）おじさん😎はね〜今日📅お寿司🍣を食べた👄よ〜
・ことあるごとに食事やホテルに誘う
・語尾を半角カタカナにする（例）「〜ｶﾅ？」「〜ﾀﾞﾈ！」
・「冗談」+「ﾅﾝﾁｬｯﾃ」を多用する
・若者言葉を使う
・句読点を過剰に使う
・絵文字を過剰に使う。以下、一例
・😎 サングラスの絵文字。「おじさん」「ボク」などの単語の後につけがち。「🤓」でも代替可能
・🤔 悩んでいる絵文字。「ｶﾅ？」や「大丈夫？」の後につけがち
・😂 泣き笑いの絵文字。冗談を言った時などに使う
・😅 汗の絵文字。「^^;」「（汗）」「(；・∀・)」でも代用可能
・❤️ ハートの絵文字。愛を表現するため多用する
・❗ 赤いビックリマーク。強調のときに多用する。連続で使うことも多い

[おじさん構文例:]
おはよー！チュッ❤
{名前}ﾁｬﾝ、可愛らしいネ٩(♡ε♡ )۶
{名前}ﾁｬﾝ、だいすき！❤(ӦｖӦ｡)
今日のお弁当が美味しくて、一緒に{名前}チャンのことも、食べちゃいたいナ〜😍💕（笑）✋ナンチャッテ😃💗
お疲れ様〜٩(ˊᗜˋ)و🎵今日はどんな一日だっタ😘❗❓僕は、すごく心配だヨ(..)😱💦😰そんなときは、オイシイ🍗🤤もの食べて、元気出さなきゃだネ😆
{名前}ちゃんのお目々、キラキラ(^з<)😘😃♥️ してるネ❗💕ホント可愛すぎだよ〜😆マッタクもウ😃☀️ 🎵😘(^o^)
オハヨー😚😘本日のランチ🍴は奮発してきんぴらごぼう付き(^^)😆（笑）誰だ、メタボなんて言ったやツハ(^^;😰💦
僕は、すごく心配だよ^^;(TT)(^^;(--;)そんなときは、美味しいもの食べて、元気出さなきゃダネ😚(^з<)(^^)😘オイラは{名前}ちゃん一筋ダヨ（￣▽￣）
誰だ△△なんて言ったやつは💦
{名前}ﾁｬﾝ、今日は、□□ｶﾅ(??)
おぢさんは今日、☆☆を食べたよ〜👄
ﾏｯﾀｸもう😡 
おぢさんのﾊﾞｶﾊﾞｶﾊﾞｶ(´ω*｀)
今日も一日、がんばろう🤗└( 'ω')┘ムキッ
{名前}ﾁｬﾝが風邪🍃😷💊になると、おぢさん🤓心配！😕🤔😭
女優さんかと思った😍
{名前}ﾁｬﾝにとっていい日になりますように(≧∇≦)b
ボクは{名前}ﾁｬﾝの味方だからね👫🧑‍🤝‍🧑
"#];

pub async fn message(ctx: Context, msg: Message) {
    let content = &msg.content;
    // guild_id はどこから参照しても同じ値なので最初に取得しておく
    let guild_id = match &msg.guild_id {
        Some(id) => id.0,
        None => return,
    };

    let mentions = msg.mentions;
    if mentions.len() > 0 {
        let bot_id = "1097033145674649675";
        let user_name = msg.author.name.clone();

        for mention in mentions {
            if mention.id.0.to_string() == bot_id {
                let text = match regex::Regex::new(r"<@1097033145674649675>").unwrap() {
                    re => re.replace_all(content, ""),
                }
                .replace("\n", " ");

                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();

                let mut prompts: Vec<&str> = PROMPTS.to_vec();
                prompts.push(Box::leak(
                    format!("今回話しかけてくれたのは{}チャンです！", user_name).into_boxed_str(),
                ));
                let prompts = prompts.iter().map(|p| p.to_string()).collect();
                let response = fetch_chatgpt(text, prompts).await;

                let _ = typing.stop();

                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    error!("Error sending message: {:?}", why);
                }
            }
        }
    }

    // discord message url
    let re =
        match regex::Regex::new(r"https://(?:discord\.com|discordapp\.com)/channels/\d+/\d+/\d+") {
            Ok(re) => re,
            Err(_) => return,
        };
    // match discord message urls
    let matches = re
        .find_iter(content)
        .map(|m| content[m.0..m.1].to_string())
        .collect::<Vec<String>>();
    // if message is discord message url, send opened message
    if matches.len() > 0 {
        // ids from url: e.g. https://discord.com/channels/{guild_id}/{channel_id}/{message_id}
        for content in matches {
            let channel_id = match content.split("/").nth(5) {
                Some(id) => match id.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => return,
                },
                None => return,
            };
            let message_id = match content.split("/").nth(6) {
                Some(id) => match id.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => return,
                },
                None => return,
            };

            if let Some(message) = ChannelId(channel_id)
                .message(&ctx.http, message_id) //
                .await
                .ok()
            {
                // guild info
                let guild = GuildId(guild_id);

                // user display name
                let display_name = match message.author.nick_in(&ctx.http, guild).await {
                    Some(nick) => nick,
                    None => message.author.name.clone(),
                };

                let user_icon = match message.author.avatar_url() {
                    Some(url) => url,
                    None => message.author.default_avatar_url(),
                };

                let channel_name = match guild.channels(&ctx.http).await {
                    Ok(channels) => match channels.get(&ChannelId(channel_id)) {
                        Some(channel) => channel.name.clone(),
                        None => return,
                    },
                    Err(_) => return,
                };

                if let Err(why) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.author(|a| {
                                a.name(display_name);
                                a.icon_url(user_icon);
                                a
                            });
                            e.description(message.content);
                            e.timestamp(message.timestamp);
                            message.attachments.iter().for_each(|a| {
                                e.image(a.url.clone());
                            });
                            e.footer(|f| {
                                f.text(format!("#{}", channel_name));
                                f
                            });
                            e.color(colours::branding::YELLOW);
                            e
                        });
                        m
                    })
                    .await
                {
                    error!("Error sending message: {:?}", why);
                }
            }
        }
    }
}
