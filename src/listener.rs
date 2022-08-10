use poise::serenity_prelude as serenity;

use crate::{Data, Error};
mod channel_create;
mod channel_delete;
mod channel_update;
mod guild_member_removal;
mod thread_create;

#[allow(dead_code)]
struct Listener<'a> {
    ctx: &'a serenity::Context,
    fwctx: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
}

pub async fn process<'a>(
    ctx: &'a serenity::Context,
    event: &'a poise::Event<'a>,
    fwctx: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> Result<(), Error> {
    let listener = Listener { ctx, fwctx, data };

    use poise::Event::*;
    match event {
        GuildMemberRemoval {
            guild_id: guild,
            user,
            member_data_if_available: member,
        } => {
            listener.guild_member_removal(guild, user, member).await?;
        }
        ThreadCreate { thread } => {
            listener.thread_create(thread).await?;
        }
        ChannelCreate { channel } => {
            listener.channel_create(channel).await?;
        }
        ChannelUpdate { old, new } => {
            listener.channel_update(old, new).await?;
        }
        ChannelDelete { channel } => {
            listener.channel_delete(channel).await?;
        }
        _ => {}
    }
    Ok(())
}
