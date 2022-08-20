use poise::serenity_prelude as serenity;

use crate::{Data, Error};
mod channel_create;
mod channel_delete;
mod channel_update;
mod guild_member_removal;
mod message;
mod message_delete;
mod message_delete_bulk;
mod message_update;
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
            listener.guild_member_removal(guild, user, member).await;
        }
        ThreadCreate { thread } => {
            listener.thread_create(thread).await;
        }
        ChannelCreate { channel } => {
            listener.channel_create(channel).await;
        }
        ChannelUpdate { old, new } => {
            listener.channel_update(old, new).await;
        }
        ChannelDelete { channel } => {
            listener.channel_delete(channel).await;
        }
        Message { new_message } => {
            listener.message(new_message).await;
        }
        MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            listener
                .message_delete(channel_id, deleted_message_id, guild_id)
                .await;
        }
        MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id,
        } => {
            listener
                .message_delete_bulk(channel_id, multiple_deleted_messages_ids, guild_id)
                .await
        }
        MessageUpdate {
            old_if_available,
            new,
            event,
        } => listener.message_update(old_if_available, new, event).await,
        _ => {}
    }
    Ok(())
}
