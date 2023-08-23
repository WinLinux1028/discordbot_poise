use super::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_delete(self, channel: &serenity::GuildChannel) {
        if let Some(globalchat) = &self.data.globalchat {
            if globalchat.is_globalchat(channel.guild_id, channel.id).await {
                let _ = globalchat.unset_channel(self.ctx, channel.guild_id).await;
            }
        }
    }
}
