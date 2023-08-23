use super::GlobalChat;

use poise::serenity_prelude as serenity;

impl GlobalChat {
    pub async fn is_globalchat(
        &self,
        guild: serenity::GuildId,
        channel: serenity::ChannelId,
    ) -> bool {
        let webhook = self.webhook.read().await;

        let globalchat_channel = webhook.get(&guild);
        let (globalchat_channel, _) = match globalchat_channel {
            Some(s) => s,
            None => return false,
        };

        globalchat_channel == &channel
    }
}
