use crate::listener::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message(self, new_message: &serenity::Message) {
        if new_message.author.bot {
            return;
        }

        if let Some(globalchat) = &self.data.globalchat {
            if self.data.is_muted(new_message.author.id).await {
                let _ = new_message.delete(self.ctx).await;
            } else {
                let _ = globalchat.send_msg(self.ctx, new_message).await;
            }
        }
    }
}
