use crate::listener::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message_delete(
        self,
        channel_id: &serenity::ChannelId,
        deleted_message_id: &serenity::MessageId,
        _guild_id: &Option<serenity::GuildId>,
    ) {
        if let Some(deleted_message) = self.ctx.cache.message(channel_id, deleted_message_id) {
            crate::features::backup::backup(
                self.ctx,
                self.data,
                &deleted_message,
                0xFF0000,
                "削除",
            )
            .await;
        }
    }
}
