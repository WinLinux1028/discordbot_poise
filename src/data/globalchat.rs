mod collect_webhook;
mod is_globalchat;
mod send_msg;
mod set_unset_channel;

use std::collections::HashMap;
use tokio::sync::RwLock;

use poise::serenity_prelude as serenity;

pub struct GlobalChat {
    pub name: String,
    webhook: RwLock<HashMap<serenity::GuildId, (serenity::ChannelId, serenity::Webhook)>>,
}

impl GlobalChat {
    pub async fn new(name: String, ctx: &serenity::Context) -> Self {
        let globalchat = GlobalChat {
            name,
            webhook: RwLock::const_new(HashMap::new()),
        };
        let _ = globalchat.collect_webhooks(ctx).await;

        globalchat
    }
}
