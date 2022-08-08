use crate::{Context, Error};

pub async fn process(ctx: Context<'_>) -> Result<bool, Error> {
    // グローバルチャットのチャンネルではコマンドを実行しない
    if crate::globalchat::is_globalchat(ctx.discord(), ctx.data(), ctx.channel_id()).await {
        return Ok(false);
    };

    Ok(true)
}
