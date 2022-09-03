use crate::{Context, Error};

use ::serenity::cache::FromStrAndCache;
use poise::serenity_prelude::{self as serenity, Mentionable};

#[poise::command(slash_command, guild_only, subcommands("give_role"))]
pub async fn member_manager(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// ロールを付与するembedを作成します
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
async fn give_role(
    ctx: Context<'_>,
    #[description = "複数選択を許すか否か"] allow_multiple: bool,
    #[description = "ロールを指定する(複数指定可)"] roles: String,
) -> Result<(), Error> {
    let roles = roles
        .split_inclusive('>')
        .map(|role| role.trim())
        .filter_map(|role| serenity::RoleId::from_str(ctx.discord(), role).ok())
        .map(|role| role.mention());
    // 0x1F1E6は🇦の文字コード､🇦〜🇿まで文字コード上で並んでいる
    let alphabet_emojis = (0..26)
        .map(|num| 0x1F1E6 + num)
        .filter_map(std::char::from_u32);

    // 絵文字とロールの対を選択肢とする
    let choices = alphabet_emojis.zip(roles);
    let choices_str = choices
        .clone()
        .map(|(emoji, role)| format!("{}: {}", emoji, role))
        .collect::<Vec<String>>()
        .join("\n");

    // 送信するembed
    let mut embeds = vec![serenity::CreateEmbed::default()];
    let embed = &mut embeds[0];
    embed.title("ロール付与").description(choices_str);
    if allow_multiple {
        embed.color(0xFFFF00);
    } else {
        embed.color(0x00FFFF);
    }

    // embedを送信
    let handle = ctx
        .send(|msg| {
            msg.embeds = embeds;
            msg
        })
        .await?
        .into_message()
        .await?;

    // リアクションを付ける
    for (emoji, _) in choices {
        handle
            .channel_id
            .create_reaction(
                ctx.discord(),
                handle.id,
                serenity::ReactionType::Unicode(format!("{}", emoji)),
            )
            .await?;
    }

    Ok(())
}
