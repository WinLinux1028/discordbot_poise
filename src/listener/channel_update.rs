use super::Listener;

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_update(self, _old: &Option<serenity::Channel>, new: &serenity::Channel) {
        if let serenity::Channel::Guild(new) = new {
            if let Some(globalchat) = &self.data.globalchat {
                if globalchat.is_globalchat(new.guild_id, new.id).await
                    && new.name() != globalchat.name
                {
                    let _ = globalchat.unset_channel(self.ctx, new.guild_id).await;
                } else if new.name() == globalchat.name {
                    let _ = globalchat.set_channel(self.ctx, new).await;
                }
            }
        }
    }
}
