use crate::listener::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message(self, new_message: &serenity::Message) {
        if new_message.author.bot || self.data.is_muted(new_message.author.id).await {
            return;
        }

        if let Some(globalchat) = &self.data.globalchat {
            let _ = globalchat.send_msg(self.ctx, new_message).await;
            return;
        }

        if let Ok(true) = new_message.mentions_me(self.ctx).await {
            let _ = new_message.channel_id.say(self.ctx, "呼んだ?").await;
        }
    }
}
