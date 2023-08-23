use crate::{Context, Error};

pub async fn process(ctx: Context<'_>) -> Result<bool, Error> {
    // ミュートされている人には機能を提供しない
    if ctx.data().is_muted(ctx.author().id).await {
        return Ok(false);
    }

    if let Some(guild) = ctx.guild_id() {
        if let Some(globalchat) = &ctx.data().globalchat {
            // グローバルチャットのチャンネルではコマンドを実行しない
            if globalchat.is_globalchat(guild, ctx.channel_id()).await {
                return Ok(false);
            };
        }
    }

    Ok(true)
}
