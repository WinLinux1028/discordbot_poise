use poise::serenity_prelude as serenity;
use sqlx::Row;

use crate::data::Data;

impl Data {
    pub async fn sns_post(&self, message: serenity::Message) -> bool {
        if let Some(guild_id) = message.guild_id {
            let tokens = sqlx::query(
                "SELECT twitter_refresh, mastodon_domain, mastodon_bearer FROM sns_post WHERE AND guildid=$1 AND channelid=$2 LIMIT 1;"
            )
                .bind(guild_id.0.to_string())
                .bind(message.channel_id.0.to_string())
                .fetch_optional(&self.psql)
                .await;

            let tokens = match tokens {
                Ok(Some(s)) => s,
                _ => return false,
            };

            let mut flag = false;

            if let Ok(twitter_refresh) = tokens.try_get::<&str, _>(0) {
                let tokens = sqlx::query(
                "SELECT twitter_refresh, mastodon_domain, mastodon_bearer FROM sns_post WHERE AND guildid=$1 AND channelid=$2 LIMIT 1;"
                )
                    .bind(guild_id.0.to_string())
                    .bind(message.channel_id.0.to_string())
                    .fetch_optional(&self.psql)
                    .await;
            }

            let mastodon_domain: Option<&str> = tokens.try_get(2).ok();
            let mastodon_bearer: Option<&str> = tokens.try_get(3).ok();
        }

        false
    }
}
