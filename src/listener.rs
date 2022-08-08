use poise::serenity_prelude as serenity;

use crate::{Data, Error};
mod guild_member_removal;

pub async fn process<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    fwctx: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> Result<(), Error> {
    use poise::Event::*;

    match event {
        ThreadCreate { thread } => {
            thread.id.join_thread(&ctx).await?;
        }
        GuildMemberRemoval {
            guild_id: guild,
            user,
            member_data_if_available: member,
        } => {
            guild_member_removal::process(ctx, fwctx, data, guild, user, member).await?;
        }
        _ => {}
    }
    Ok(())
}
