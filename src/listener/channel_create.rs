use crate::{globalchat, listener::Listener};

use poise::serenity_prelude as serenity;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl Listener<'_> {
    pub async fn channel_create(self, channel: &serenity::GuildChannel) -> Result<(), Error> {
        if let Some(globalchat_name) = &self.data.globalchat_name {
            if globalchat_name == channel.name() {
                let _ = globalchat::set_channel(self.ctx, self.data, channel).await;
            }
        }

        Ok(())
    }
}
