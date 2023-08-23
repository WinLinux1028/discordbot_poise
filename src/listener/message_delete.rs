use super::Listener;
use crate::data::backup;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message_delete(
        self,
        channel: &serenity::ChannelId,
        deleted_message: &serenity::MessageId,
        _guild: &Option<serenity::GuildId>,
    ) {
        if let Some(deleted_message) = self.ctx.cache.message(channel, deleted_message) {
            backup::backup(self.ctx, self.data, &deleted_message, 0xFF0000, "削除").await;
        }
    }
}
