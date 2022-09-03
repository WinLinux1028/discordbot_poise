use tokio::io::AsyncReadExt;

use poise::serenity_prelude as serenity;
use sqlx::mysql;

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
        listener: |ctx, event, fwctx, data| Box::pin(listener::process(ctx, event, fwctx, data)),
        command_check: Some(|ctx| Box::pin(command_check::process(ctx))),
        on_error: |err| Box::pin(on_error::process(err)),
        commands: vec![
            command::owner::register(),
            command::owner::mute::mute(),
            command::general::help(),
            command::general::ping(),
            command::general::say(),
            command::general::nade(),
            command::member_manager::member_manager(),
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(data_raw.token.clone())
        .intents(serenity::GatewayIntents::all())
        .user_data_setup(|ctx, ready, framework| {
            Box::pin(ready::process(ctx, ready, framework, data_raw))
        });

    framework.run_autosharded().await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config(Vec<DataRaw>);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DataRaw {
    token: String,
    globalchat_name: Option<String>,
    mariadb: String,
    backup_id: Option<serenity::ChannelId>,
}

impl DataRaw {
    pub async fn into_data(self, ctx: &serenity::Context) -> Result<Data, Error> {
        let mut globalchat = None;
        if let Some(globalchat_name) = self.globalchat_name {
            globalchat = Some(globalchat::GlobalChat::new(globalchat_name, ctx).await);
        }

        let mariadb = mysql::MySqlPool::connect(&self.mariadb).await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mutelist (user INT8 UNSIGNED NOT NULL PRIMARY KEY);",
        )
        .execute(&mariadb)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS guildconfig (
                guild INT8 UNSIGNED NOT NULL PRIMARY KEY,
                member_manager_lockdowned BOOLEAN NOT NULL, member_manager_allowlockdown INT8 UNSIGNED, member_manager_memberrole INT8 UNSIGNED, member_manager_kickable INT8 UNSIGNED
            );",
        )
        .execute(&mariadb)
        .await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS member_manager_newmember (
                guild INT8 UNSIGNED NOT NULL, user INT8 UNSIGNED NOT NULL, jointime INT8 NOT NULL,
                PRIMARY KEY (guild, user)
            );",
        )
        .execute(&mariadb)
        .await?;

        let mut backup = None;
        if let Some(backup_id) = self.backup_id {
            if let Ok(serenity::Channel::Category(backup_category)) =
                backup_id.to_channel(ctx).await
            {
                backup = Some(backup_category)
            }
        }

        Ok(Data {
            globalchat,
            mariadb,
            backup,
        })
    }
}
