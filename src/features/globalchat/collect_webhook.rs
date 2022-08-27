use std::collections::HashMap;

use crate::Error;

use crate::features::*;
use poise::serenity_prelude as serenity;

impl globalchat::GlobalChat {
    pub async fn collect_webhooks(&self, ctx: &serenity::Context) -> Result<(), Error> {
        *self.webhook.write().await = HashMap::new();

        let botuser = ctx.cache.current_user();
        // 入っているサーバーを回す
        for guild in botuser.guilds(ctx).await? {
            let channels = match guild.id.channels(ctx).await {
                Ok(o) => o,
                Err(_) => continue,
            };

            // サーバーにあるチャンネルを回す
            for (_, channel) in channels {
                // グローバルチャットのチャンネルの場合
                if channel.name == self.name {
                    let webhooks = match channel.webhooks(ctx).await {
                        Ok(o) => o,
                        Err(_) => continue,
                    };

                    let mut globalchat_webhook = None;
                    // チャンネルのwebhookを回す
                    for webhook in webhooks {
                        let userid = match &webhook.user {
                            Some(s) => s.id,
                            None => continue,
                        };
                        // BOT自身が作ったチャンネルの場合
                        if userid == botuser.id {
                            globalchat_webhook = Some(webhook);
                            break;
                        }
                    }

                    if let Some(globalchat_webhook) = globalchat_webhook {
                        self.webhook
                            .write()
                            .await
                            .insert(guild.id, (channel.id, globalchat_webhook));
                    } else if self.set_channel(ctx, &channel).await.is_err() {
                        continue;
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}
