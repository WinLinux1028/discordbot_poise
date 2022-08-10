use crate::listener::Listener;

use poise::serenity_prelude::{self as serenity, Mentionable};
pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl Listener<'_> {
    pub async fn guild_member_removal(
        self,
        guild: &serenity::GuildId,
        user: &serenity::User,
        _member: &Option<serenity::Member>,
    ) -> Result<(), Error> {
        let _ = notify_member_removal(self.ctx, guild, user).await;

        Ok(())
    }
}

async fn notify_member_removal(
    ctx: &serenity::Context,
    guild: &serenity::GuildId,
    user: &serenity::User,
) -> Result<(), Error> {
    let guild = match guild.to_guild_cached(ctx) {
        Some(s) => s,
        None => return Ok(()),
    };
    let channel = match guild.system_channel_id {
        Some(s) => s,
        None => return Ok(()),
    };

    let message = format!(
        "{}({}#{})が退出しました",
        user.mention(),
        &user.name,
        user.discriminator
    );
    channel.say(ctx, message).await?;

    Ok(())
}
