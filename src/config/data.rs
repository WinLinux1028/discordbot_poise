use oauth2::basic::BasicClient;
use poise::serenity_prelude as serenity;
use sqlx::postgres;

pub struct Data {
    pub globalchat: Option<crate::globalchat::GlobalChat>,
    pub psql: postgres::PgPool,
    pub backup: Option<serenity::ChannelCategory>,
    pub oauth_redirect_url: Option<String>,
    pub twitter_client: Option<BasicClient>,
}

impl Data {
    pub async fn is_muted(&self, user: serenity::UserId) -> bool {
        let result = sqlx::query("SELECT * FROM mutelist WHERE userid=$1 LIMIT 1;")
            .bind(user.0.to_string())
            .fetch_optional(&self.psql)
            .await;

        match result {
            Ok(o) => o.is_some(),
            Err(_) => false,
        }
    }
}
