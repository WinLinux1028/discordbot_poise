use poise::serenity_prelude as serenity;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Backup {
    pub guild: serenity::GuildId,
    pub category: serenity::ChannelId,
}
