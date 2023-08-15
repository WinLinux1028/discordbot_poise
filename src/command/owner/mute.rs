use crate::{Context, Error};

use poise::serenity_prelude::{self as serenity, Mentionable};
use sqlx::Row;

#[allow(clippy::redundant_closure)]
#[poise::command(
    prefix_command,
    owners_only,
    hide_in_help,
    subcommands("add", "remove")
)]
pub async fn mute(ctx: Context<'_>) -> Result<(), Error> {
    let muted_users = sqlx::query("SELECT (userid) FROM mutelist")
        .fetch_all(&ctx.data().psql)
        .await?;
    let muted_users: Vec<serenity::UserId> = muted_users
        .into_iter()
        .filter_map(|user| {
            user.get::<&str, _>(0)
                .parse()
                .ok()
                .map(|u| serenity::UserId(u))
        })
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
    sqlx::query("INSERT INTO mutelist(userid) VALUES ($1)")
        .bind(user.0.to_string())
        .execute(&ctx.data().psql)
        .await?;
    ctx.say("OK").await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
pub async fn remove(ctx: Context<'_>, user: serenity::UserId) -> Result<(), Error> {
    sqlx::query("DELETE FROM mutelist WHERE userid=$1")
        .bind(user.0.to_string())
        .execute(&ctx.data().psql)
        .await?;
    ctx.say("OK").await?;

    Ok(())
}
