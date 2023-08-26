pub mod mastodon;
mod twitter;

use super::Data;
use crate::Error;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, RefreshToken, TokenResponse,
};
use poise::serenity_prelude as serenity;
use sqlx::{PgConnection, PgPool};

impl Data {
    pub async fn sns_post(&self, message: &serenity::Message) -> bool {
        let mut flag = false;

        let mastodon = mastodon::post(self, message).await;
        flag |= mastodon.is_ok();
        let mastodon_url = match mastodon {
            Ok(Some(u)) => Some(u),
            _ => None,
        };

        flag |= twitter::post(self, message, mastodon_url.as_deref())
            .await
            .is_ok();

        flag
    }
}

#[derive(sqlx::FromRow)]
pub struct Token {
    pub refresh: Option<String>,
    pub bearer: String,
    pub expires: Option<i64>,
}

impl Token {
    pub fn new(
        refresh: Option<&RefreshToken>,
        bearer: &AccessToken,
        expires_in: Option<std::time::Duration>,
    ) -> Result<Self, Error> {
        let mut result = Self {
            refresh: refresh.map(|t| t.secret().clone()),
            bearer: bearer.secret().clone(),
            expires: None,
        };

        if let Some(expires_in) = expires_in {
            result.set_expires(expires_in)?;
        }

        Ok(result)
    }

    pub async fn get_token(
        psql: &PgPool,
        guild: serenity::GuildId,
        service: &str,
        client: &BasicClient,
    ) -> Result<Token, Error> {
        let guild_str = guild.0.to_string();

        let mut token: Token = sqlx::query_as("SELECT domain, refresh, bearer, expires FROM sns_post WHERE guildid=$1 AND service=$2 LIMIT 1;")
            .bind(&guild_str)
            .bind(service)
            .fetch_optional(psql)
            .await?
            .ok_or("token not found")?;

        if let Some(expires) = token.expires {
            let refresh = token
                .refresh
                .as_ref()
                .ok_or("bearer token expires, but refresh token not found")?;

            if chrono::Local::now().timestamp() > expires {
                let new_token = client
                    .exchange_refresh_token(&RefreshToken::new(refresh.clone()))
                    .request_async(async_http_client)
                    .await?;

                token.bearer = new_token.access_token().secret().clone();
                if let Some(refresh) = new_token.refresh_token() {
                    token.refresh = Some(refresh.secret().clone());
                }
                match new_token.expires_in() {
                    Some(d) => token.set_expires(d)?,
                    None => token.expires = None,
                };

                let mut trx = psql.begin().await?;
                token.db_update(&mut trx, &guild_str, service).await?;
                trx.commit().await?;
            }
        }

        Ok(token)
    }

    pub async fn db_update(
        &self,
        psql: &mut PgConnection,
        guild: &str,
        service: &str,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE sns_post SET refresh=$1, bearer=$2, expires=$3 WHERE guildid=$4 AND service=$5;")
            .bind(&self.refresh)
            .bind(&self.bearer)
            .bind(self.expires)
            .bind(guild)
            .bind(service)
            .execute(psql)
            .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn db_insert(
        &self,
        psql: &mut PgConnection,
        guild: &str,
        channel: &str,
        domain: &str,
        service: &str,
        client_id: Option<&str>,
        client_secret: Option<&str>,
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO sns_post (guildid, service, domain, channelid, refresh, bearer, expires, client_id, client_secret)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                ON CONFLICT (guildid, service)
                DO UPDATE SET domain=$3, channelid=$4, refresh=$5, bearer=$6, expires=$7, client_id=$8, client_secret=$9;",
        )
        .bind(guild)
        .bind(service)
        .bind(domain)
        .bind(channel)
        .bind(&self.refresh)
        .bind(&self.bearer)
        .bind(self.expires)
        .bind(client_id)
        .bind(client_secret)
        .execute(psql)
        .await?;

        Ok(())
    }

    fn set_expires(&mut self, expires_in: std::time::Duration) -> Result<(), Error> {
        let now = chrono::Local::now();
        let expires_in = chrono::Duration::from_std(expires_in)?;
        self.expires = Some((now + expires_in).timestamp());

        Ok(())
    }
}
