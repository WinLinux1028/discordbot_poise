use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn is_globalchat(
    data: &Data,
    guild_id: serenity::GuildId,
    channel_id: serenity::ChannelId,
) -> bool {
    let globalchat = match &data.globalchat {
        Some(s) => s,
        None => return false,
    };
    let webhook = globalchat.webhook.read().await;

    let globalchat_channel = webhook.get(&guild_id);
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
                    if set_channel(ctx, data, &channel).await.is_err() {
                        continue;
                    }
                } else {
                    data.globalchat_webhook
                        .write()
                        .await
                        .insert(guild.id, (channel.id, globalchat_webhook.unwrap()));
                }

                break;
            }
        }
    }

    Ok(())
}

pub async fn set_channel(
    ctx: &serenity::Context,
    data: &Data,
    channel: &serenity::GuildChannel,
) -> Result<(), Error> {
    let webhook = channel.id.create_webhook(ctx, "globalchat").await?;
    let oldval = data
        .globalchat_webhook
        .write()
        .await
        .insert(channel.guild_id, (channel.id, webhook));

    let oldval = match oldval {
        Some(s) => s,
        None => return Ok(()),
    };
    oldval.1.delete(ctx).await?;

    Ok(())
}

pub async fn unset_channel(
    ctx: &serenity::Context,
    data: &Data,
    channel: &serenity::GuildChannel,
) -> Result<(), Error> {
    let oldval = data
        .globalchat_webhook
        .write()
        .await
        .remove(&channel.guild_id);

    let oldval = match oldval {
        Some(s) => s,
        None => return Ok(()),
    };
    oldval.1.delete(ctx).await?;

    Ok(())
}

pub async fn send_msg(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    let guild = match &message.guild_id {
        Some(s) => s,
        None => return Ok(()),
    };
    if !is_globalchat(data, *guild, message.channel_id).await {
        return Ok(());
    }

    // 画像をembedとして扱う
    let embeds = message
        .attachments
        .iter()
        .filter(|file| file.width.is_some() && file.height.is_some())
        .map(|file| serenity::Embed::fake(|embed| embed.image(&file.proxy_url)))
        .collect();
    // 各種情報を取得
    let avator = match message.author.static_avatar_url() {
        Some(s) => s,
        None => message.author.face(),
    };
    let guild_name = match guild.name(ctx) {
        Some(s) => s,
        None => return Ok(()),
    };
    // 各サーバーに送信する内容を決める
    let mut content = serenity::ExecuteWebhook::default();
    content
        .avatar_url(avator)
        .username(format!("{}@{}", message.author.tag(), guild_name))
        .content(message.content_safe(ctx))
        .embeds(embeds);

    let webhooks = data.globalchat_webhook.read().await;
    // メッセージが送信されたチャンネル以外を仕分け､Webhookの情報だけを残す
    let webhooks_iter = webhooks
        .values()
        .filter(|webhook| webhook.0 != message.channel_id)
        .map(|webhook| webhook.1.clone());
    // 各サーバーに送信していく
    for webhook in webhooks_iter {
        let ctx = Arc::clone(&ctx.http);
        let content = content.clone();
        // 高速化のためマルチスレッド化
        tokio::spawn(async move {
            let _ = webhook
                .execute(ctx, false, |execute| {
                    *execute = content;
                    execute
                })
                .await;
        });
    }

    Ok(())
}
