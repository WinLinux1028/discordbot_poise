use std::collections::HashMap;

use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn is_globalchat(
    data: &Data,
    guild_id: serenity::GuildId,
    channel_id: serenity::ChannelId,
) -> bool {
    let data = data.globalchat_webhook.read().await;

    let globalchat_channel = data.get(&guild_id);
    let (globalchat_channel, _) = match globalchat_channel {
        Some(s) => s,
        None => return false,
    };

    globalchat_channel == &channel_id
}

pub async fn collect_webhooks(ctx: &serenity::Context, data: &Data) -> Result<(), Error> {
    *data.globalchat_webhook.write().await = HashMap::new();
    let globalchat_name = match &data.globalchat_name {
        Some(s) => s,
        None => return Ok(()),
    };

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
            if &channel.name == globalchat_name {
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
                // グローバルチャットのチャンネルが見つからなかった場合
                if globalchat_webhook.is_none() {
                    globalchat_webhook = match channel.id.create_webhook(ctx, "globalchat").await {
                        Ok(o) => Some(o),
                        Err(_) => {
                            continue;
                        }
                    };
                }

                data.globalchat_webhook
                    .write()
                    .await
                    .insert(guild.id, (channel.id, globalchat_webhook.unwrap()));

                break;
            }
        }
    }

    Ok(())
}
