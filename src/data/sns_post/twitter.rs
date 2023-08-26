use super::Token;
use crate::{data::Data, Error};

use poise::serenity_prelude as serenity;
use twitter_text::extractor::{Extract, ValidatingExtractor};
use twitter_v2::{authorization, TwitterApi};

pub async fn post(
    data: &Data,
    message: &serenity::Message,
    mastodon_url: Option<&str>,
) -> Result<(), Error> {
    let guild = message.guild_id.ok_or("")?;
    let client = data.twitter_client.as_ref().ok_or("")?;

    let token = Token::get_token(&data.psql, guild, "Twitter", client).await?;
    let api = TwitterApi::new(authorization::BearerToken::new(token.bearer));

    let (mut len, mut text) = cut(message.content.trim(), mastodon_url);
    for i in &message.attachments {
        let after_len = len + twitter_text_config::default().transformed_url_length + 1;
        if after_len <= 280 {
            text.push('\n');
            text.push_str(&i.url);
        }
        len = after_len;
    }

    api.post_tweet().text(text).send().await?;

    Ok(())
}

fn cut(s: &str, mastodon_url: Option<&str>) -> (i32, String) {
    let mut extractor = ValidatingExtractor::new(twitter_text_config::default());
    let s = extractor.prep_input(s);

    let result = extractor
        .extract_urls_with_indices(s.as_str())
        .parse_results;

    if !result.is_valid {
        if let Some(mastodon_url) = mastodon_url {
            let s = format!("全文: {}\n{}", mastodon_url, s);
            return cut(&s, None);
        }
    }

    let start: usize = result.valid_text_range.start().try_into().unwrap();
    let end: usize = result.valid_text_range.end().try_into().unwrap();

    let s: Vec<u16> = s.encode_utf16().collect();
    let s = char::decode_utf16(s[start..=end].iter().copied())
        .filter_map(|c| c.ok())
        .collect();

    (result.weighted_length, s)
}
