use crate::{Context, Error};

pub async fn process(ctx: Context<'_>) -> Result<bool, Error> {
    if let Some(guild_id) = ctx.guild_id() {
        // グローバルチャットのチャンネルではコマンドを実行しない
        if crate::globalchat::is_globalchat(ctx.data(), guild_id, ctx.channel_id()).await {
            return Ok(false);
        };
    }

    Ok(true)
}
