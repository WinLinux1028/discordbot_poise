use crate::{globalchat, listener::Listener};

use poise::serenity_prelude as serenity;

impl Listener<'_> {
    pub async fn channel_update(self, _old: &Option<serenity::Channel>, new: &serenity::Channel) {
        if let serenity::Channel::Guild(new) = new {
            if let Some(globalchat_name) = &self.data.globalchat_name {
                if globalchat::is_globalchat(self.data, new.guild_id, new.id).await
                    && new.name() != globalchat_name
                {
                    let _ = globalchat::unset_channel(self.ctx, self.data, new).await;
                } else if new.name() == globalchat_name {
                    let _ = globalchat::set_channel(self.ctx, self.data, new).await;
                }
            }
        }
    }
}
