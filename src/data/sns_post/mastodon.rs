use std::io::Cursor;

use super::Token;
use crate::{data::Data, Error};

use megalodon::{
    entities::UploadMedia,
    mastodon::Mastodon,
    megalodon::{PostStatusInputOptions, PostStatusOutput},
    Megalodon,
};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use poise::serenity_prelude as serenity;

pub async fn post(data: &Data, message: &serenity::Message) -> Result<Option<String>, Error> {
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

    let token =
        Token::get_token(&data.psql, guild, message.channel_id, "Mastodon", &client).await?;
    let api = Mastodon::new(format!("https://{}", domain), Some(token.bearer), None);

    let text: String = message
        .content
        .trim()
        .chars()
        .zip(0..500)
        .map(|(i, _)| i)
        .collect();

    let mut option = PostStatusInputOptions::default();
    let media_ids = get_media_ids(&api, message).await;
    if !media_ids.is_empty() {
        option.media_ids = Some(media_ids);
    }

    let toot = api.post_status(text, Some(&option)).await?;
    let status = match toot.json {
        PostStatusOutput::Status(s) => s,
        _ => return Ok(None),
    };
    let url = match status.url {
        Some(u) => u,
        None => return Ok(None),
    };

    Ok(Some(url))
}

async fn get_media_ids(api: &impl Megalodon, message: &serenity::Message) -> Vec<String> {
    let mut attachments = Vec::new();
    for i in message.attachments.iter() {
        let content_type = match i.content_type.as_ref() {
            Some(s) => s,
            None => continue,
        };

        if content_type.starts_with("image/") {
            attachments.push(i);

            if attachments.len() == 4 {
                break;
            }
        } else if attachments.is_empty()
            && (content_type.starts_with("audio/") || content_type.starts_with("video/"))
        {
            attachments.push(i);
            break;
        }
    }

    let mut media_ids = Vec::new();
    for i in attachments {
        let attachment = match i.download().await {
            Ok(o) => Cursor::new(o),
            Err(_) => continue,
        };

        let media = match api.upload_media_reader(Box::new(attachment), None).await {
            Ok(o) => o,
            Err(_) => continue,
        };

        if let UploadMedia::Attachment(a) = media.json {
            media_ids.push(a.id)
        } else if let UploadMedia::AsyncAttachment(a) = media.json {
            for _ in 0..10 {
                match api.get_media(a.id.clone()).await {
                    Ok(o) => {
                        media_ids.push(o.json.id);
                        break;
                    }
                    Err(_) => tokio::time::sleep(std::time::Duration::from_secs(10)).await,
                };
            }
        }
    }

    media_ids
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
