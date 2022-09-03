use crate::Error;

use crate::features::*;
use poise::serenity_prelude as serenity;

impl globalchat::GlobalChat {
    pub async fn set_channel(
        &self,
        ctx: &serenity::Context,
        channel: &serenity::GuildChannel,
    ) -> Result<(), Error> {
        // チャンネルにWebhookが既にないか確認する
        let mut webhook = channel
            .webhooks(ctx)
            .await?
            .into_iter()
            .filter(|webhook| webhook.user.is_some())
            .find(|webhook| webhook.user.as_ref().unwrap().id == ctx.cache.current_user().id);

        // もしWebhookが存在しなければ新しく作る
        if webhook.is_none() {
            webhook = Some(channel.id.create_webhook(ctx, "globalchat").await?);
        }
        let webhook = webhook.unwrap();
        let webhook_id = webhook.id;

        // 取得したWebhookを登録する
        let oldval = self
            .webhook
            .write()
            .await
            .insert(channel.guild_id, (channel.id, webhook));

        // 古いWebhookの後始末
        if let Some(oldval) = oldval {
            if oldval.1.id != webhook_id {
                let _ = oldval.1.delete(ctx).await;
            }
        }

        Ok(())
    }

    pub async fn unset_channel(
        &self,
        ctx: &serenity::Context,
        guild: serenity::GuildId,
    ) -> Result<(), Error> {
        let oldval = self.webhook.write().await.remove(&guild);

        if let Some(oldval) = oldval {
            oldval.1.delete(ctx).await?;
        }

        Ok(())
    }
}
