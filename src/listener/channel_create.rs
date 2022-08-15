use crate::{globalchat, listener::Listener};

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_create(self, channel: &serenity::GuildChannel) {
        if let Some(globalchat_name) = &self.data.globalchat_name {
            if globalchat_name == channel.name() {
                let _ = globalchat::set_channel(self.ctx, self.data, channel).await;
            }
        }
    }
}
