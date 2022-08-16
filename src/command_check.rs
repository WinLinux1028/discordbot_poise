use crate::{Context, Error};

pub async fn process(ctx: Context<'_>) -> Result<bool, Error> {
    if let Some(guild_id) = ctx.guild_id() {
        if let Some(globalchat) = &ctx.data().globalchat {
            // グローバルチャットのチャンネルではコマンドを実行しない
            if globalchat.is_globalchat(guild_id, ctx.channel_id()).await {
                return Ok(false);
            };
        }
    }

    Ok(true)
}
