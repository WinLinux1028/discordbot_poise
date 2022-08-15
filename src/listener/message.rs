use crate::{globalchat, listener::Listener};

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message(self, new_nessage: &serenity::Message) {
        if new_nessage.author.bot {
            return;
        }

        let _ = globalchat::send_msg(self.ctx, self.data, new_nessage).await;
    }
}
