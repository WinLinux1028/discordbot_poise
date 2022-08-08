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
    if let Some(guild) = guild.to_guild_cached(ctx) {
        if let Some(channel) = guild.system_channel_id {
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
        };
    };
    Ok(())
}
