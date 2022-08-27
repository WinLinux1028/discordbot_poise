use crate::listener::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_create(self, channel: &serenity::GuildChannel) {
        if let Some(globalchat) = &self.data.globalchat {
            if globalchat.name == channel.name() {
                let _ = globalchat.set_channel(self.ctx, channel).await;
            }
        }
    }
}
