use super::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn message(self, new_message: &serenity::Message) {
        if new_message.author.bot || self.data.is_muted(new_message.author.id).await {
            return;
        }

        if let Some(globalchat) = &self.data.globalchat {
            if let Some(guild) = new_message.guild_id {
                if globalchat
                    .is_globalchat(guild, new_message.channel_id)
                    .await
                {
                    let _ = globalchat.send_msg(self.ctx, new_message).await;
                    return;
                }
            }
        }

        if let Some(s) = new_message.attachments.get(0) {
            match &s.content_type {
                Some(s) => println!("{}", s),
                None => println!("no content type"),
            }
        }

        if self.data.sns_post(new_message).await {
            return;
        }

        if new_message.content.contains('🍎') {
            let _ = new_message.channel_id.say(self.ctx, "🍏").await;
            return;
        }

        if let Ok(true) = new_message.mentions_me(self.ctx).await {
            let _ = new_message.channel_id.say(self.ctx, "呼んだ?").await;
        }
    }
}
