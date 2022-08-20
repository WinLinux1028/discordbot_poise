use poise::serenity_prelude as serenity;

use crate::Data;
mod build_message;
mod send_category;
mod send_guild;

pub async fn backup(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
    color: u32,
    action: &str,
) {
    let guild = match message.guild(ctx) {
        Some(s) => s,
        None => return,
    };

    // 送信する内容
    let send = match build_message::process(ctx, message, color, action).await {
        Some(s) => s,
        None => return,
    };

    // そのメッセージがあったサーバーの履歴チャンネルに送信
    let _ = send_guild::process(ctx, action, &guild, send.clone()).await;

    // 設定ファイルに指定されたカテゴリー内のチャンネルに送信
    let _ = send_category::process(ctx, data, send.clone()).await;
}
