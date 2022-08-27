use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn process(
    ctx: &serenity::Context,
    data: &Data,
    guild: &serenity::Guild,
    send: serenity::CreateMessage<'_>,
) -> Result<(), Error> {
    let backup = match &data.backup {
        Some(s) => s,
        None => return Ok(()),
    };
    let backup_guild = match backup.guild_id.to_guild_cached(ctx) {
        Some(s) => s,
        None => return Ok(()),
    };

    // バックアップカテゴリに属している､かつサーバーのIDで終わっているチャンネルを見つける
    let mut backup_channel = backup_guild
        .channels
        .iter()
        .filter_map(|(_, channel)| match channel {
            serenity::Channel::Guild(g) => Some(g),
            _ => None,
        })
        .filter(|channel| channel.parent_id.is_some())
        .filter(|channel| unsafe { channel.parent_id.unwrap_unchecked() } == backup.id)
        .find(|channel| channel.name.ends_with(&format!("_{}", guild.id.0)));

    // チャンネルが見つからなければ作る
    let new_channel;
    if backup_channel.is_none() {
        new_channel = backup_guild
            .create_channel(ctx, |channel| {
                channel
                    .kind(serenity::ChannelType::Text)
                    .name(format!("{}_{}", &guild.name, guild.id.0))
                    .category(backup.id)
            })
            .await?;
        backup_channel = Some(&new_channel);
    }

    // 送信する
    let backup_channel = match backup_channel {
        Some(s) => s,
        None => return Ok(()),
    };
    backup_channel
        .send_message(ctx, |msg| {
            *msg = send;
            msg
        })
        .await?;

    Ok(())
}
