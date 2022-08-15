use crate::{globalchat, listener::Listener};

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_delete(self, channel: &serenity::GuildChannel) {
        if globalchat::is_globalchat(self.data, channel.guild_id, channel.id).await {
            let _ = globalchat::unset_channel(self.ctx, self.data, channel).await;
        }
    }
}
