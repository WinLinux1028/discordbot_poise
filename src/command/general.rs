use crate::{Context, Error};

use tokio::time::Instant;

use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "入力されたコマンドの詳細を表示"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

/// 応答速度を測定できるpingコマンド
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let handle = ctx.say("計測中...").await?;
    let end = start.elapsed().as_secs_f64();
    handle
        .edit(ctx, |reply| {
            reply.content(format!("pong!\n応答時間は{}秒です", end))
        })
        .await?;

    Ok(())
}

/// Botに任意の発言をさせる
#[poise::command(slash_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "BOTに発言させたい文章"] mut content: String,
) -> Result<(), Error> {
    content = serenity::content_safe(
        ctx.serenity_context(),
        content,
        &serenity::ContentSafeOptions::default(),
        &[],
    );
    ctx.say(content).await?;

    Ok(())
}

/// 霊夢を撫でる
#[poise::command(slash_command)]
pub async fn nade(
    ctx: Context<'_>,
    #[description = "色を英語(小文字)で指定する｡ デフォルトは赤"] color: Option<String>,
) -> Result<(), Error> {
    let url;
    if let Some(color) = color {
        match color.as_str() {
            "red" => url = "https://media.discordapp.net/attachments/840924639223349250/879655572281106432/index.gif",
            "blue" => url = "https://media.discordapp.net/attachments/840924639223349250/879655365522911262/index.gif",
            "green" => url = "https://media.discordapp.net/attachments/840924639223349250/879655924258717717/index.gif",
            "yellow" | "gold" => url = "https://media.discordapp.net/attachments/907217028031578127/940637756642689074/index.gif",
            "white" => url = "https://media.discordapp.net/attachments/840924639223349250/923831999892955156/index.gif",
            "black" => url = "https://cdn.discordapp.com/attachments/945894119513931846/945894299889958942/index.gif",
            "ukraine" => url = "https://cdn.discordapp.com/attachments/945894119513931846/946312974715469824/index.gif",
            _ => url = "この色はありません"
        }
    } else {
        url = "https://media.discordapp.net/attachments/840924639223349250/879655572281106432/index.gif"
    }

    ctx.say(url).await?;

    Ok(())
}

/// 緑霊夢のニックネームを変更する
#[poise::command(slash_command, guild_only)]
pub async fn rename(
    ctx: Context<'_>,
    #[description = "新しいニックネーム"] name: Option<String>,
) -> Result<(), Error> {
    let guild = match ctx.guild_id() {
        Some(s) => s,
        None => return Ok(()),
    };

    guild
        .edit_nickname(ctx.serenity_context(), name.as_deref())
        .await?;
    ctx.say("変更しました").await?;

    Ok(())
}
