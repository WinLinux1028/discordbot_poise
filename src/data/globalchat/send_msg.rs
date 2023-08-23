use super::GlobalChat;
use crate::Error;

use std::sync::Arc;

use poise::serenity_prelude as serenity;

impl GlobalChat {
    pub async fn send_msg(
        &self,
        ctx: &serenity::Context,
        message: &serenity::Message,
    ) -> Result<(), Error> {
        // 画像をembedとして扱う
        let embeds = message
            .attachments
            .iter()
            .filter(|file| file.width.is_some() && file.height.is_some())
            .map(|file| serenity::Embed::fake(|embed| embed.image(&file.url)))
            .collect();
        // 各種情報を取得
        let avator = match message.author.static_avatar_url() {
            Some(s) => s,
            None => message.author.face(),
        };
        let guild_name = match message.guild_id.unwrap().name(ctx) {
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

        let webhooks = self.webhook.read().await;
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
}
