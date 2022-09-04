use crate::{
    config::{guild_config::GuildConfig, member_manager::NewMember},
    listener::Listener,
    Error,
};

use poise::serenity_prelude::{self as serenity, Mentionable};

impl Listener<'_> {
    pub async fn guild_member_removal(
        self,
        guild: &serenity::GuildId,
        user: &serenity::User,
        _member: &Option<serenity::Member>,
    ) {
        if user.id == self.ctx.cache.current_user().id {
            let _ = GuildConfig::remove(&self.data.mariadb, *guild).await;
            let _ = NewMember::remove_guild(&self.data.mariadb, *guild);
            return;
        }

        let _ = notify_member_removal(self.ctx, guild, user).await;
        let _ = NewMember::remove(&self.data.mariadb, *guild, user.id).await;
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

    let message = format!("{}({})が退出しました", user.mention(), user.tag());
    channel.say(ctx, message).await?;

    Ok(())
}
