use super::GlobalChat;
use crate::Error;

use std::collections::HashMap;

use poise::serenity_prelude as serenity;
use tokio_stream::StreamExt;

impl GlobalChat {
    pub async fn collect_webhooks(&self, ctx: &serenity::Context) -> Result<(), Error> {
        *self.webhook.write().await = HashMap::new();

        // Botが入っているサーバーとそのチャンネルの対を作る
        let guilds = tokio_stream::iter(ctx.cache.current_user().guilds(ctx).await?)
            .then(|guild| async move { (guild.id.channels(ctx).await, guild) })
            .filter(|(channels, _)| channels.is_ok());
        tokio::pin!(guilds);

        while let Some((channels, _guild)) = guilds.next().await {
            // チャンネル名がグローバルチャットのものと同じものに絞る
            let channels = channels
                .unwrap()
                .into_values()
                .filter(|channel| channel.name == self.name);

            for channel in channels {
                if self.set_channel(ctx, &channel).await.is_err() {
                    continue;
                }
            }
        }

        Ok(())
    }
}
