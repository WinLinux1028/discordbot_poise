use poise::serenity_prelude as serenity;

use crate::features::*;

impl globalchat::GlobalChat {
    pub async fn is_globalchat(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> bool {
        let webhook = self.webhook.read().await;

        let globalchat_channel = webhook.get(&guild_id);
        let (globalchat_channel, _) = match globalchat_channel {
            Some(s) => s,
            None => return false,
        };

        globalchat_channel == &channel_id
    }
}
