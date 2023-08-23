use super::Listener;
use crate::data::backup;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message_update(
        self,
        old_if_available: &Option<serenity::Message>,
        new: &Option<serenity::Message>,
        _event: &serenity::MessageUpdateEvent,
    ) {
        let old = match old_if_available {
            Some(s) => s,
            None => return,
        };
        let new = match new {
            Some(s) => s,
            None => return,
        };

        // BOT自身が編集したものは無視する
        if old.author.id == self.ctx.cache.current_user().id {
            return;
        }
        // URLのembedが付いたときには処理しないようにする
        if old.edited_timestamp == new.edited_timestamp {
            return;
        }

        backup::backup(self.ctx, self.data, old, 0x00FF00, "編集").await;
    }
}
