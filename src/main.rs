use tokio::io::AsyncReadExt;

use poise::serenity_prelude as serenity;
use sqlx::mysql;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod command;
use command::*;
pub mod features;
use features::*;

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
            owner::register(),
            owner::mute::mute(),
            general::help(),
            general::ping(),
            general::say(),
            general::nade(),
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
    pub async fn to_data(self, ctx: &serenity::Context) -> Result<Data, Error> {
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

pub struct Data {
    globalchat: Option<globalchat::GlobalChat>,
    mariadb: mysql::MySqlPool,
    backup: Option<serenity::ChannelCategory>,
}

impl Data {
    pub async fn is_muted(&self, user: serenity::UserId) -> bool {
        let result = sqlx::query("SELECT (user) FROM mutelist WHERE user=? LIMIT 1")
            .bind(user.0)
            .fetch_optional(&self.mariadb)
            .await;

        match result {
            Ok(o) => o.is_some(),
            Err(_) => false,
        }
    }
}
