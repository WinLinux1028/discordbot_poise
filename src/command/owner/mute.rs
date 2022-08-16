use crate::{Context, Error};

use poise::serenity_prelude::{self as serenity, Mentionable};
use sqlx::Row;

#[poise::command(
    prefix_command,
    owners_only,
    hide_in_help,
    subcommands("add", "remove")
)]
pub async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    let muted_users = sqlx::query("SELECT user FROM mutelist")
        .fetch_all(&ctx.data().mariadb)
        .await?;
    let muted_users: Vec<serenity::UserId> = muted_users
        .into_iter()
        .map(|user| serenity::UserId(user.get(0)))
        .collect();

    let mut send = String::new();
    for user in muted_users {
        send.push_str(&format!("{}\n", user.mention()));
    }
    ctx.say(send).await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
pub async fn add(ctx: Context<'_>, user: serenity::UserId) -> Result<(), Error> {
    sqlx::query("INSERT INTO mutelist (user) VALUES (?)")
        .bind(user.0)
        .execute(&ctx.data().mariadb)
        .await?;
    ctx.say("OK").await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
pub async fn remove(ctx: Context<'_>, user: serenity::UserId) -> Result<(), Error> {
    sqlx::query("DELETE FROM mutelist WHERE user=? LIMIT 1")
        .bind(user.0)
        .execute(&ctx.data().mariadb)
        .await?;
    ctx.say("OK").await?;

    Ok(())
}
