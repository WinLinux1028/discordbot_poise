#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::time::Duration;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};
use tokio::{io::AsyncReadExt, time};

use poise::serenity_prelude as serenity;
use sqlx::postgres;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod command;
pub mod features;
use features::*;

use crate::config::data::Data;
pub mod config;

mod command_check;
mod listener;
mod on_error;
mod ready;

#[tokio::main]
async fn main() {
    let mut config = String::new();
    tokio::fs::File::open("config.json")
        .await
        .unwrap()
        .read_to_string(&mut config)
        .await
        .unwrap();
    let config: Config = serde_json::from_str(&config).unwrap();

    for data_raw in config.0 {
        tokio::spawn(new_bot(data_raw));
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(u64::MAX)).await;
    }
}

async fn new_bot(data_raw: DataRaw) {
    let options = poise::FrameworkOptions {
        event_handler: |ctx, event, fwctx, data| {
            Box::pin(listener::process(ctx, event, fwctx, data))
        },
        command_check: Some(|ctx| Box::pin(command_check::process(ctx))),
        on_error: |err| Box::pin(on_error::process(err)),
        commands: vec![
            command::owner::register(),
            command::owner::mute::mute(),
            command::general::help(),
            command::general::ping(),
            command::general::say(),
            command::general::nade(),
            command::general::rename(),
            command::sns_post::sns_post(),
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(data_raw.token.clone())
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, ready, framework| Box::pin(ready::process(ctx, ready, framework, data_raw)));

    framework.run_autosharded().await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config(Vec<DataRaw>);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataRaw {
    token: String,
    globalchat_name: Option<String>,
    psql: String,
    backup_id: Option<serenity::ChannelId>,
    oauth_redirect_url: Option<String>,
    twitter_client_id: Option<String>,
    twitter_client_secret: Option<String>,
}

impl DataRaw {
    pub async fn into_data(self, ctx: &serenity::Context) -> Result<Data, Error> {
        let mut globalchat = None;
        if let Some(globalchat_name) = self.globalchat_name {
            globalchat = Some(globalchat::GlobalChat::new(globalchat_name, ctx).await);
        }

        // DBに接続
        let mut psql;
        loop {
            let mut result = true;
            if let Ok(o) = postgres::PgPool::connect(&self.psql).await {
                psql = o;

                result &=
                    sqlx::query("CREATE TABLE IF NOT EXISTS mutelist (userid TEXT PRIMARY KEY);")
                        .execute(&psql)
                        .await
                        .is_ok();
                result &=
                    sqlx::query("CREATE TABLE IF NOT EXISTS oauth2_auth (state TEXT PRIMARY KEY, guildid TEXT UNIQUE NOT NULL, channelid TEXT NOT NULL, service TEXT NOT NULL, code_verifier TEXT NOT NULL, expired BIGINT NOT NULL);")
                        .execute(&psql)
                        .await
                        .is_ok();
                result &=
                    sqlx::query("CREATE TABLE IF NOT EXISTS oauth2_refresh (service TEXT, refresh TEXT, bearer TEXT NOT NULL, PRIMARY KEY(service, refresh));")
                        .execute(&psql)
                        .await
                        .is_ok();
                result &=
                    sqlx::query("CREATE TABLE IF NOT EXISTS sns_post (guildid TEXT, channelid TEXT NOT NULL, twitter_refresh TEXT, mastodon_domain TEXT, mastodon_bearer TEXT, PRIMARY KEY(guildid));")
                        .execute(&psql)
                        .await
                        .is_ok();

                if result {
                    break;
                }
            };
            time::sleep(Duration::from_secs(1)).await;
            continue;
        }

        // DBのクリーンアップ
        let psql2 = psql.clone();
        tokio::spawn(async move {
            let psql = psql2;
            loop {
                time::sleep(Duration::from_secs(60 * 3)).await;
                let time = chrono::Local::now().timestamp();

                let _ = sqlx::query("DELETE FROM oauth2_auth WHERE expired<=$1;")
                    .bind(time)
                    .execute(&psql)
                    .await;
            }
        });

        let mut backup = None;
        if let Some(backup_id) = self.backup_id {
            if let Ok(serenity::Channel::Category(backup_category)) =
                backup_id.to_channel(ctx).await
            {
                backup = Some(backup_category)
            }
        }

        let oauth_redirect_url = self.oauth_redirect_url.clone();
        let mut twitter_client = None;
        if let Some(oauth_redirect_url) = self.oauth_redirect_url {
            if let Some(twitter_client_id) = self.twitter_client_id {
                twitter_client = Some(
                    BasicClient::new(
                        ClientId::new(twitter_client_id),
                        self.twitter_client_secret.map(ClientSecret::new),
                        AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string())?,
                        Some(TokenUrl::new(
                            "https://api.twitter.com/2/oauth2/token".to_string(),
                        )?),
                    )
                    .set_revocation_uri(RevocationUrl::new(
                        "https://api.twitter.com/2/oauth2/revoke".to_string(),
                    )?)
                    .set_redirect_uri(RedirectUrl::new(oauth_redirect_url)?),
                );
            }
        }

        Ok(Data {
            globalchat,
            psql,
            backup,
            oauth_redirect_url,
            twitter_client,
        })
    }
}
