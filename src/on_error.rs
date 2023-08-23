use crate::{Context, Data, Error};

pub async fn process(err: poise::FrameworkError<'_, Data, Error>) {
    use poise::FrameworkError::*;
    match err {
        Command { error, ctx } => {
            let _ = ctx
                .say(format!("コマンド実行中にエラーが発生しました:\n{}", error))
                .await;
        }

        ArgumentParse {
            error,
            input: _,
            ctx,
        } => {
            let _ = ctx
                .say(format!(
                    "コマンドの入力形式に間違いがあるようです:\n{}",
                    error
                ))
                .await;
        }

        MissingUserPermissions {
            missing_permissions: Some(perm),
            ctx,
        } => {
            let _ = ctx
                .say(format!("あなたの権限が足りません\n足りない権限: {}", perm))
                .await;
        }

        NotAnOwner { ctx } => {
            let _ = ctx.say("このコマンドはBOT管理者専用です").await;
        }

        GuildOnly { ctx } => {
            let _ = ctx.say("このコマンドはサーバー専用です").await;
        }

        DmOnly { ctx } => {
            let _ = ctx.say("このコマンドはDM専用です").await;
        }

        NsfwOnly { ctx } => {
            let _ = ctx.say("このコマンドはNSFWチャンネル専用です").await;
        }

        MissingBotPermissions {
            missing_permissions: perm,
            ctx,
        } => {
            let _ = ctx
                .say(format!("BOTの権限が足りません\n足りない権限: {}", perm))
                .await;
        }

        CommandStructureMismatch {
            description: desc,
            ctx,
        } => {
            let _ = Context::from(ctx)
                .say(format!("BOTにバグが発生しているようです:\n{}", desc))
                .await;
        }

        CooldownHit {
            remaining_cooldown,
            ctx,
        } => {
            let _ = Context::from(ctx)
                .say(format!(
                    "{}秒後に再試行してください",
                    remaining_cooldown.as_secs()
                ))
                .await;
        }

        _ => {}
    }
}
