use crate::Error;
use poise::serenity_prelude as serenity;

#[derive(sqlx::FromRow)]
struct GuildConfigSQL {
    guild: u64,
    member_manager_lockdowned: bool,
    member_manager_allowlockdown: Option<u64>,
    member_manager_memberrole: Option<u64>,
    member_manager_kickable: Option<u64>,
}

impl From<GuildConfig> for GuildConfigSQL {
    fn from(item: GuildConfig) -> Self {
        let member_manager_memberrole = item.member_manager_memberrole.map(|role| role.0);

        Self {
            guild: item.guild.0,
            member_manager_lockdowned: item.member_manager_lockdowned,
            member_manager_allowlockdown: item.member_manager_allowlockdown,
            member_manager_memberrole,
            member_manager_kickable: item.member_manager_kickable,
        }
    }
}

#[derive(Clone)]
pub struct GuildConfig {
    pub guild: serenity::GuildId,
    pub member_manager_lockdowned: bool,
    pub member_manager_allowlockdown: Option<u64>,
    pub member_manager_memberrole: Option<serenity::RoleId>,
    pub member_manager_kickable: Option<u64>,
}

impl GuildConfig {
    pub async fn new(mariadb: &sqlx::mysql::MySqlPool, guild: serenity::GuildId) -> Self {
        let guild_config = Self {
            guild,
            member_manager_lockdowned: false,
            member_manager_allowlockdown: None,
            member_manager_memberrole: None,
            member_manager_kickable: None,
        };

        let raw: GuildConfigSQL = guild_config.clone().into();
        let _ = sqlx::query(
            "INSERT INTO guildconfig
                (
                    guild,
                    member_manager_lockdowned, member_manager_allowlockdown, member_manager_memberrole, member_manager_kickable
                )
                VALUES (?, ?, ?, ?, ?);"
            )
            .bind(raw.guild)
            .bind(raw.member_manager_lockdowned)
            .bind(raw.member_manager_allowlockdown)
            .bind(raw.member_manager_memberrole)
            .bind(raw.member_manager_kickable)
            .execute(mariadb).await;

        guild_config
    }

    pub async fn get(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
    ) -> Result<Option<Self>, Error> {
        let raw: Option<GuildConfigSQL> =
            sqlx::query_as("SELECT * FROM guildconfig WHERE guild=? LIMIT 1;")
                .bind(guild.0)
                .fetch_optional(mariadb)
                .await?;

        let guild_config = raw.map(|raw| raw.into());
        Ok(guild_config)
    }

    pub async fn write_back(self, mariadb: &sqlx::mysql::MySqlPool) -> Result<(), Error> {
        let raw: GuildConfigSQL = self.into();
        sqlx::query("UPDATE guildconfig
            SET member_manager_lockdowned=?, member_manager_allowlockdown=?, member_manager_memberrole=?, member_manager_kickable=?
            WHERE guild=? LIMIT 1;"
        )
            .bind(raw.member_manager_lockdowned)
            .bind(raw.member_manager_allowlockdown)
            .bind(raw.member_manager_memberrole)
            .bind(raw.member_manager_kickable)
            .bind(raw.guild)
            .execute(mariadb)
            .await?;

        Ok(())
    }

    pub async fn remove(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM guildconfig WHERE guild=? LIMIT 1;")
            .bind(guild.0)
            .execute(mariadb)
            .await?;

        Ok(())
    }
}

impl From<GuildConfigSQL> for GuildConfig {
    fn from(item: GuildConfigSQL) -> Self {
        let member_manager_memberrole = item.member_manager_memberrole.map(serenity::RoleId);

        Self {
            guild: serenity::GuildId(item.guild),
            member_manager_lockdowned: item.member_manager_lockdowned,
            member_manager_allowlockdown: item.member_manager_allowlockdown,
            member_manager_memberrole,
            member_manager_kickable: item.member_manager_kickable,
        }
    }
}
