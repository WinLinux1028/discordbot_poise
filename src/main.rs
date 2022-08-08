use std::collections::HashMap;
use tokio::{io::AsyncReadExt, sync::RwLock};

use poise::serenity_prelude as serenity;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod command;
use command::*;

mod command_check;
mod globalchat;
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
        tokio::spawn(new_bot(Data::convert(data_raw).await.unwrap()));
    }

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(u64::MAX)).await;
    }
}

async fn new_bot(data: Data) {
    let options = poise::FrameworkOptions {
        listener: |ctx, event, fwctx, data| Box::pin(listener::process(ctx, event, fwctx, data)),
        command_check: Some(|ctx| Box::pin(command_check::process(ctx))),
        on_error: |err| Box::pin(on_error::process(err)),
        commands: vec![
            owner::register(),
            general::help(),
            general::ping(),
            general::say(),
            general::nade(),
        ],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(&data.token)
        .intents(serenity::GatewayIntents::all())
        .user_data_setup(|ctx, ready, framework| {
            Box::pin(ready::process(ctx, ready, framework, data))
        });

    framework.run_autosharded().await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config(Vec<DataRaw>);

#[derive(serde::Serialize, serde::Deserialize)]
struct DataRaw {
    token: String,
    globalchat_name: Option<String>,
}

pub struct Data {
    token: String,
    globalchat_name: Option<String>,
    globalchat_webhook:
        RwLock<HashMap<serenity::GuildId, (serenity::ChannelId, serenity::Webhook)>>,
}

impl Data {
    async fn convert(from: DataRaw) -> Result<Data, Error> {
        Ok(Data {
            token: from.token,
            globalchat_name: from.globalchat_name,
            globalchat_webhook: RwLock::const_new(HashMap::new()),
        })
    }
}
