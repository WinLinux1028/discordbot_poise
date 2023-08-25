use crate::{data::Data, Error};

use poise::serenity_prelude as serenity;
use twitter_v2::{authorization, TwitterApi};

pub async fn post(data: &Data, message: &serenity::Message) -> Result<(), Error> {
    let guild = message.guild_id.ok_or("")?;
    let client = data.twitter_client.as_ref().ok_or("")?;

    let token = super::get_token(&data.psql, guild, message.channel_id, "Twitter", client).await?;
    let api = TwitterApi::new(authorization::BearerToken::new(token.bearer));

    let mut text = message.content.clone();
    text.push('\n');
    for i in &message.attachments {
        text.push_str(&i.proxy_url);
        text.push('\n');
    }

    api.post_tweet().text(text).send().await?;

    Ok(())
}
