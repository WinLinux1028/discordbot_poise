use poise::serenity_prelude as serenity;
use sqlx::mysql;

pub struct Data {
    pub globalchat: Option<crate::globalchat::GlobalChat>,
    pub mariadb: mysql::MySqlPool,
    pub backup: Option<serenity::ChannelCategory>,
}

impl Data {
    pub async fn is_muted(&self, user: serenity::UserId) -> bool {
        let result = sqlx::query("SELECT * FROM mutelist WHERE user=? LIMIT 1;")
            .bind(user.0)
            .fetch_optional(&self.mariadb)
            .await;

        match result {
            Ok(o) => o.is_some(),
            Err(_) => false,
        }
    }
}
