use crate::{Context, Error};

use oauth2::{AuthorizationRequest, CsrfToken, PkceCodeChallenge, Scope};

#[poise::command(
    slash_command,
    prefix_command,
    guild_only,
    required_permissions = "MANAGE_CHANNELS",
    default_member_permissions = "MANAGE_CHANNELS",
    subcommands("twitter", "mastodon")
)]
pub async fn sns_post(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(
    prefix_command,
    owners_only,
    hide_in_help,
    subcommands("twitter_set", "twitter_disable")
)]
pub async fn twitter(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, rename = "set", guild_cooldown = 360)]
pub async fn twitter_set(ctx: Context<'_>) -> Result<(), Error> {
    if let Some(twitter_client) = &ctx.data().twitter_client {
        let oauth = twitter_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("tweet.read".to_string()))
            .add_scope(Scope::new("tweet.write".to_string()))
            .add_scope(Scope::new("users.read".to_string()))
            .add_scope(Scope::new("offline.access".to_string()));

        set(ctx, "twitter.com", oauth).await
    } else {
        ctx.say("Twitter連携は無効です").await?;
        Ok(())
    }
}

#[poise::command(prefix_command, rename = "disable", guild_cooldown = 360)]
pub async fn twitter_disable(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild_id().unwrap().0.to_string();
    let mut trx = ctx.data().psql.begin().await?;

    sqlx::query("DELETE FROM oauth2_refresh WHERE service='twitter.com' AND refresh=(SELECT twitter_refresh FROM sns_post WHERE guildid=$1);")
        .bind(&guild)
        .execute(&mut *trx)
        .await?;
    sqlx::query("UPDATE sns_post SET twitter_refresh=NULL WHERE guildid=$1;")
        .bind(&guild)
        .execute(&mut *trx)
        .await?;

    trx.commit().await?;

    ctx.say("無効化しました").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn mastodon(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub async fn set(
    ctx: Context<'_>,
    hostname: &str,
    oauth: AuthorizationRequest<'_>,
) -> Result<(), Error> {
    let guild = ctx.guild_id().unwrap().0.to_string();
    let channel = ctx.channel_id().0.to_string();

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, state) = oauth.set_pkce_challenge(pkce_challenge).url();

    let time = chrono::Local::now().timestamp();
    sqlx::query("INSERT INTO oauth2_auth(state, guildid, channelid, service, code_verifier, expired) VALUES ($1, $2, $3, $4, $5, $6);")
        .bind(state.secret())
        .bind(&guild)
        .bind(&channel)
        .bind(hostname)
        .bind(pkce_verifier.secret())
        .bind(time + 60 * 3)
        .execute(&ctx.data().psql)
        .await?;

    ctx.say(format!("ここで認証を行ってください:\n{}", url))
        .await?;

    Ok(())
}
