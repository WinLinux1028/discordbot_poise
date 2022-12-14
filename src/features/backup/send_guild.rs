use poise::serenity_prelude as serenity;

use crate::Error;

pub async fn process(
    ctx: &serenity::Context,
    action: &str,
    guild: &serenity::Guild,
    send: serenity::CreateMessage<'_>,
) -> Result<(), Error> {
    let history_channel = guild
        .channels
        .values()
        .filter_map(|channel| match channel {
            serenity::Channel::Guild(g) => Some(g),
            _ => None,
        })
        .find(|channel| channel.name() == format!("{}履歴", action));

    let history_channel = match history_channel {
        Some(s) => s,
        None => return Ok(()),
    };

    history_channel
        .send_message(ctx, |msg| {
            *msg = send;
            msg
        })
        .await?;

    Ok(())
}
