use crate::{Context, Error};

pub async fn process(ctx: Context<'_>) -> Result<bool, Error> {
    // グローバルチャットのチャンネルではコマンドを実行しない
    if let Some(globalchat_name) = &ctx.data().globalchat_name {
        if let Some(channel_name) = ctx.channel_id().name(ctx.discord()).await {
            if globalchat_name == &channel_name {
                return Ok(false);
            }
        }
    }

    Ok(true)
}
