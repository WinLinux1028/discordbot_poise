use crate::listener::Listener;

use poise::serenity_prelude as serenity;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl Listener<'_> {
    pub async fn channel_create(self, channel: &serenity::GuildChannel) -> Result<(), Error> {
        Ok(())
    }
}
