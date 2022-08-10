use crate::listener::Listener;

use poise::serenity_prelude as serenity;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl Listener<'_> {
    pub async fn thread_create(self, thread: &serenity::GuildChannel) -> Result<(), Error> {
        let _ = thread.id.join_thread(self.ctx).await;

        Ok(())
    }
}
