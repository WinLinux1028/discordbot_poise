mod mastodon;
mod twitter;

use crate::Error;

use super::Data;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, RefreshToken, TokenResponse,
};
use poise::serenity_prelude as serenity;
use sqlx::{PgConnection, PgPool};

impl Data {
    pub async fn sns_post(&self, message: &serenity::Message) {}
}

async fn get_token(
    psql: &PgPool,
    guild: serenity::GuildId,
    service: &str,
    client: &BasicClient,
) -> Result<Token, Error> {
    let guild_str = guild.0.to_string();

    let mut token: Token = sqlx::query_as("SELECT domain, channelid, refresh, bearer, expires FROM sns_post WHERE guildid=$1 AND service=$2 LIMIT 1;")
            .bind(&guild_str)
            .bind(service)
            .fetch_optional(psql)
            .await?
            .ok_or("token not found")?;

    if let Some(expires) = token.expires {
        let refresh = token
            .refresh
            .as_ref()
            .ok_or("bearer token is expired, but refresh token not found")?;

        if chrono::Local::now().timestamp() > expires {
            let new_token = client
                .exchange_refresh_token(&RefreshToken::new(refresh.clone()))
                .request_async(async_http_client)
                .await?;

            token.bearer = new_token.access_token().secret().clone();
            if let Some(refresh) = new_token.refresh_token() {
                token.refresh = Some(refresh.secret().clone());
            }
            token.expires = match new_token.expires_in().map(chrono::Duration::from_std) {
                Some(d) => Some((chrono::Local::now() + d?).timestamp()),
                None => None,
            };

            let mut trx = psql.begin().await?;
            token.db_insert(&mut trx, &guild_str, service).await?;
            trx.commit().await?;
        }
    }

    Ok(token)
}

#[derive(sqlx::FromRow)]
pub struct Token {
    pub domain: String,
    pub channelid: String,
    pub refresh: Option<String>,
    pub bearer: String,
    pub expires: Option<i64>,
}

impl Token {
    pub fn new(
        domain: String,
        channelid: String,
        refresh: Option<&RefreshToken>,
        bearer: &AccessToken,
        expires_in: Option<std::time::Duration>,
    ) -> Result<Self, Error> {
        let mut result = Self {
            domain,
            channelid,
            refresh: refresh.map(|t| t.secret().clone()),
            bearer: bearer.secret().clone(),
            expires: None,
        };

        if let Some(expires_in) = expires_in {
            result.set_expires(expires_in)?;
        }

        Ok(result)
    }

    pub fn set_expires(&mut self, expires_in: std::time::Duration) -> Result<(), Error> {
        let now = chrono::Local::now();
        let expires_in = chrono::Duration::from_std(expires_in)?;
        self.expires = Some((now + expires_in).timestamp());

        Ok(())
    }

    pub async fn db_insert(
        &self,
        psql: &mut PgConnection,
        guild: &str,
        service: &str,
    ) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO sns_post (guildid, service, domain, channelid, refresh, bearer, expires)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (guildid, service)
                DO UPDATE SET refresh=$5, bearer=$6, expires=$7;",
        )
        .bind(guild.to_string())
        .bind(service)
        .bind(&self.domain)
        .bind(&self.channelid)
        .bind(&self.refresh)
        .bind(&self.bearer)
        .bind(self.expires)
        .execute(psql)
        .await?;

        Ok(())
    }
}
