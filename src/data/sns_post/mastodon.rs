use super::Token;
use crate::{data::Data, Error};

use megalodon::{mastodon::Mastodon, megalodon::PostStatusInputOptions, Megalodon};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use poise::serenity_prelude as serenity;

pub async fn post(data: &Data, message: &serenity::Message) -> Result<(), Error> {
    let guild = message.guild_id.ok_or("")?;

    let (domain, client_id, client_secret): (String, String, String) =
        sqlx::query_as("SELECT domain, client_id, client_secret FROM sns_post WHERE guildid=$1 AND channelid=$2 AND service='Mastodon' LIMIT 1;")
            .bind(guild.0.to_string())
            .bind(message.channel_id.0.to_string())
            .fetch_optional(&data.psql)
            .await?
            .ok_or("token not found")?;
    let client = get_client(
        data.hostname.as_ref().ok_or("")?,
        &domain,
        client_id,
        client_secret,
    )?;

    let token = Token::get_token(
        &data.psql,
        guild,
        message.channel_id,
        &domain,
        "Mastodon",
        &client,
    )
    .await?;
    let api = Mastodon::new(format!("https://{}", domain), Some(token.bearer), None);

    let text: String = message
        .content
        .trim()
        .chars()
        .zip(0..500)
        .map(|(i, _)| i)
        .collect();

    let option = PostStatusInputOptions::default();
    // let attachments = Vec::new();
    // for i in message.attachments {
    //     content_type = match i.content_type {
    //         Some(c) => c,
    //         None => continue,
    //     };
    // }

    api.post_status(text, Some(&option)).await?;

    Ok(())
}

pub fn get_client(
    bot_hostname: &str,
    domain: &str,
    client_id: String,
    client_secret: String,
) -> Result<BasicClient, Error> {
    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(format!("https://{}/oauth/authorize", domain))?,
        Some(TokenUrl::new(format!("https://{}/oauth/token", domain))?),
    )
    .set_revocation_uri(RevocationUrl::new(format!(
        "https://{}/oauth/revoke",
        domain
    ))?)
    .set_redirect_uri(RedirectUrl::new(format!(
        "https://{}/oauth",
        &bot_hostname
    ))?);

    Ok(client)
}
