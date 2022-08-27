use crate::Error;

use crate::features::*;
use poise::serenity_prelude as serenity;

impl globalchat::GlobalChat {
    pub async fn set_channel(
        &self,
        ctx: &serenity::Context,
        channel: &serenity::GuildChannel,
    ) -> Result<(), Error> {
        let webhook = channel.id.create_webhook(ctx, "globalchat").await?;
        let oldval = self
            .webhook
            .write()
            .await
            .insert(channel.guild_id, (channel.id, webhook));

        // 古いWebhookの後始末
        let oldval = match oldval {
            Some(s) => s,
            None => return Ok(()),
        };
        oldval.1.delete(ctx).await?;

        Ok(())
    }

    pub async fn unset_channel(
        &self,
        ctx: &serenity::Context,
        guild: serenity::GuildId,
    ) -> Result<(), Error> {
        let oldval = self.webhook.write().await.remove(&guild);

        let oldval = match oldval {
            Some(s) => s,
            None => return Ok(()),
        };
        oldval.1.delete(ctx).await?;

        Ok(())
    }
}
