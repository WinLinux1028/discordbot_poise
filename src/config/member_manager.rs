use crate::Error;
use chrono::TimeZone;
use poise::serenity_prelude as serenity;

#[derive(sqlx::FromRow)]
struct NewMemberSQL {
    guild: u64,
    user: u64,
    jointime: u64,
}

impl TryFrom<NewMember> for NewMemberSQL {
    type Error = Error;

    fn try_from(item: NewMember) -> Result<Self, Self::Error> {
        let jointime = u64::try_from(item.jointime.timestamp())?;

        Ok(Self {
            guild: item.guild.0,
            user: item.user.0,
            jointime,
        })
    }
}

#[derive(Clone)]
pub struct NewMember {
    pub guild: serenity::GuildId,
    pub user: serenity::UserId,
    pub jointime: chrono::DateTime<chrono::offset::Local>,
}

impl NewMember {
    async fn new(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
        user: serenity::UserId,
    ) -> Result<Self, Error> {
        let new_member = Self {
            guild,
            user,
            jointime: chrono::offset::Local::now(),
        };

        // DBにエントリーを追加
        let raw: NewMemberSQL = new_member.clone().try_into().unwrap();
        sqlx::query(
            "INSERT INTO member_manager_newmember (guild, user, jointime) VALUES (?, ?, ?)",
        )
        .bind(raw.guild)
        .bind(raw.user)
        .bind(raw.jointime)
        .execute(mariadb)
        .await?;

        Ok(new_member)
    }

    pub async fn get(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
        user: serenity::UserId,
    ) -> Result<Self, Error> {
        // DBから取得する
        let raw: Option<NewMemberSQL> = sqlx::query_as(
            "SELECT * FROM member_manager_newmember WHERE guild=? AND user=? LIMIT 1;",
        )
        .bind(guild.0)
        .bind(user.0)
        .fetch_optional(mariadb)
        .await?;

        let mut new_member = None;
        if let Some(new_member2) = raw {
            // 変換に成功したらその値をnew_memberに入れ､失敗したらDBのエントリーを削除
            if let Ok(new_member2) = new_member2.try_into() {
                new_member = Some(new_member2);
            } else {
                Self::remove(mariadb, guild, user).await?;
            }
        };

        // DBから見つからなかったor変換に失敗したら再作成
        if new_member.is_none() {
            new_member = Some(Self::new(mariadb, guild, user).await?)
        }

        Ok(new_member.unwrap())
    }

    pub async fn write_back(self, mariadb: &sqlx::mysql::MySqlPool) -> Result<(), Error> {
        let raw: NewMemberSQL = self.try_into()?;
        sqlx::query(
            "UPDATE member_manager_newmember SET jointime=? WHERE guild=? AND user=? LIMIT 1;",
        )
        .bind(raw.jointime)
        .bind(raw.guild)
        .bind(raw.user)
        .execute(mariadb)
        .await?;

        Ok(())
    }

    pub async fn remove(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
        user: serenity::UserId,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM member_manager_newmember WHERE guild=? AND user=? LIMIT 1;")
            .bind(guild.0)
            .bind(user.0)
            .execute(mariadb)
            .await?;

        Ok(())
    }

    pub async fn remove_guild(
        mariadb: &sqlx::mysql::MySqlPool,
        guild: serenity::GuildId,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM member_manager_newmember WHERE guild=? LIMIT 1;")
            .bind(guild.0)
            .execute(mariadb)
            .await?;

        Ok(())
    }
}

impl TryFrom<NewMemberSQL> for NewMember {
    type Error = Error;

    fn try_from(item: NewMemberSQL) -> Result<Self, Self::Error> {
        let jointime = i64::try_from(item.jointime)?;
        let jointime = chrono::offset::Local.timestamp(jointime, 0);

        Ok(Self {
            guild: serenity::GuildId(item.guild),
            user: serenity::UserId(item.user),
            jointime,
        })
    }
}
