use super::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn thread_create(self, thread: &serenity::GuildChannel) {
        let _ = thread.id.join_thread(self.ctx).await;
    }
}
