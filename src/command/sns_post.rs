use crate::{Context, Error};

use megalodon::{mastodon::Mastodon, megalodon::AppInputOptions, Megalodon};
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope};

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
    let client = match ctx.data().twitter_client.as_ref() {
        Some(c) => c,
        None => return Err("Twitter連携機能が無効になっています".into()),
    };

    let (pkce_challenge, code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, state) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
        .add_scope(Scope::new("offline.access".to_string()))
        .url();

    set(&ctx, "Twitter", "twitter.com", &state, &code_verifier).await?;
    ctx.say(format!("ここで認証してください:\n{}", url.as_str()))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, rename = "disable", guild_cooldown = 360)]
pub async fn twitter_disable(ctx: Context<'_>) -> Result<(), Error> {
    disable(&ctx, "Twitter").await
}

#[poise::command(slash_command, subcommands("mastodon_set", "mastodon_disable"))]
pub async fn mastodon(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, rename = "set", guild_cooldown = 360)]
pub async fn mastodon_set(ctx: Context<'_>, domain: String) -> Result<(), Error> {
    let bot_hostname = ctx.data().hostname.clone().ok_or("")?;
    let api = Mastodon::new(format!("https://{}", domain), None, None);

    let app_config = AppInputOptions {
        scopes: Some(vec![
            "read".to_string(),
            "write".to_string(),
            "push".to_string(),
            "admin:read".to_string(),
            "admin:write".to_string(),
        ]),
        redirect_uris: Some(format!("https://{}/oauth", &bot_hostname)),
        website: None,
    };

    let app = api
        .create_app(
            format!("{}", &ctx.serenity_context().cache.current_user().name),
            &app_config,
        )
        .await?;

    let client = crate::data::sns_post::mastodon::get_client(
        &bot_hostname,
        &domain,
        app.client_id,
        app.client_secret,
    )?;

    let (pkce_challenge, code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (url, state) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("write:statuses".to_string()))
        .add_scope(Scope::new("write:media".to_string()))
        .url();

    set(&ctx, "Mastodon", &domain, &state, &code_verifier).await?;
    ctx.say(format!("ここで認証してください:\n{}", url.as_str()))
        .await?;

    Ok(())
}

#[poise::command(slash_command, rename = "disable", guild_cooldown = 360)]
pub async fn mastodon_disable(ctx: Context<'_>) -> Result<(), Error> {
    disable(&ctx, "Mastodon").await
}

async fn set(
    ctx: &Context<'_>,
    service: &str,
    domain: &str,
    state: &CsrfToken,
    code_verifier: &PkceCodeVerifier,
) -> Result<(), Error> {
    let guild = match ctx.guild_id() {
        Some(g) => g,
        None => return Err("サーバー内でのみ実行できます".into()),
    };

    sqlx::query(
        "INSERT INTO
        oauth2_state (state, guildid, channelid, service, domain, code_verifier, expires)
        VALUES ($1, $2, $3, $4, $5, $6, $7);",
    )
    .bind(state.secret())
    .bind(guild.0.to_string())
    .bind(ctx.channel_id().0.to_string())
    .bind(service)
    .bind(domain)
    .bind(code_verifier.secret())
    .bind(chrono::Local::now().timestamp() + 60 * 3)
    .execute(&ctx.data().psql)
    .await?;

    Ok(())
}

async fn disable(ctx: &Context<'_>, service: &str) -> Result<(), Error> {
    let guild = match ctx.guild_id() {
        Some(g) => g,
        None => return Err("サーバー内でのみ実行できます".into()),
    };

    sqlx::query("DELETE FROM sns_post WHERE guildid=$1 AND service=$2;")
        .bind(guild.0.to_string())
        .bind(service)
        .execute(&ctx.data().psql)
        .await?;

    ctx.say("無効化しました").await?;

    Ok(())
}
