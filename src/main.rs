use poise::serenity_prelude as serenity;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod command;
use command::*;
use tokio::io::AsyncReadExt;

mod error;
mod listener;
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
        on_error: |err| Box::pin(error::on_error(err)),
        commands: vec![
            owner::register(),
            general::help(),
            general::ping(),
            general::say(),
            general::nade(),
        ],
        listener: |ctx, event, fwctx, data| Box::pin(listener::listener(ctx, event, fwctx, data)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(&data.token)
        .intents(serenity::GatewayIntents::all())
        .user_data_setup(|ctx, ready, framework| {
            Box::pin(ready::ready(ctx, ready, framework, data))
        });

    framework.run_autosharded().await.unwrap();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Config(Vec<DataRaw>);

#[derive(serde::Serialize, serde::Deserialize)]
struct DataRaw {
    token: String,
}

pub struct Data {
    token: String,
}

impl Data {
    async fn convert(from: DataRaw) -> Result<Data, Error> {
        Ok(Data { token: from.token })
    }
}
