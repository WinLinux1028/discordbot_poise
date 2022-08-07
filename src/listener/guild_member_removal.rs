use poise::serenity_prelude::{self as serenity, Mentionable};

use crate::Data;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn process(
    ctx: &serenity::Context,
    _fwctx: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    guild: &serenity::GuildId,
    user: &serenity::User,
    _member: &Option<serenity::Member>,
) -> Result<(), Error> {
    let guild = guild.to_partial_guild(ctx).await?;
    let channel = match guild.system_channel_id {
        Some(s) => s,
        None => return Ok(()),
    };
    channel
        .say(
            ctx,
            format!(
                "{}({}#{})が退出しました",
                user.mention(),
                &user.name,
                user.discriminator
            ),
        )
        .await?;
    Ok(())
}
