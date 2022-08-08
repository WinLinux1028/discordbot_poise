use crate::{Data, Error};
use poise::serenity_prelude as serenity;

pub async fn process<'a>(
    ctx: &'a serenity::Context,
    ready: &'a serenity::Ready,
    framework: &'a poise::Framework<Data, Error>,
    data: Data,
) -> Result<Data, Error> {
    ctx.idle().await;

    let _ = crate::globalchat::collect_webhooks(ctx, &data).await;

    ctx.set_presence(
        Some(serenity::Activity::streaming(
            "Made by Rust",
            "https://www.youtube.com/watch?v=2uquJXO4scY",
        )),
        serenity::OnlineStatus::Online,
    )
    .await;

    println!("logged in as {}", &ready.user.name);
    for i in &framework.options().owners {
        if let Ok(i) = i.create_dm_channel(ctx).await {
            let _ = i.say(ctx, "起動完了").await;
        }
    }

    Ok(data)
}
