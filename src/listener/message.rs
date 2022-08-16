use crate::listener::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message(self, new_nessage: &serenity::Message) {
        if new_nessage.author.bot {
            return;
        }

        if let Some(globalchat) = &self.data.globalchat {
            let _ = globalchat.send_msg(self.ctx, new_nessage).await;
        }
    }
}
