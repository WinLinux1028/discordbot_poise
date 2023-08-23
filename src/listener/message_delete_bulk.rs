use super::Listener;
use crate::data::backup;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message_delete_bulk(
        self,
        channel: &serenity::ChannelId,
        multiple_deleted_messages_ids: &Vec<serenity::MessageId>,
        _guild: &Option<serenity::GuildId>,
    ) {
        for deleted_message_id in multiple_deleted_messages_ids {
            if let Some(deleted_message) = self.ctx.cache.message(channel, deleted_message_id) {
                backup::backup(self.ctx, self.data, &deleted_message, 0xFF0000, "削除").await;
            }
        }
    }
}
