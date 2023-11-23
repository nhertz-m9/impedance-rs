use std::sync::Arc;
use chrono::Local;
use poise::serenity_prelude::Mention;
use crate::{
    helpers::{
        Context,
        ApplicationError,
        reply_ephemeral, 
        get_songbird
    }, 
    constants::APPLICATION_COLOUR
};


#[poise::command(slash_command, guild_only)]
pub async fn connect(ctx: Context<'_>) -> Result<(), ApplicationError> {
    let guild = ctx.guild().unwrap();

    let connect_to = 
        match guild.voice_states.get(&ctx.author().id).and_then(|s| s.channel_id) {
            Some(channel_id) => channel_id,
            None => {
                reply_ephemeral(ctx, "Should be called in the voice channel your are in.").await?;
                return Ok(());
            }
        };

    if connect_to != ctx.channel_id() {
        reply_ephemeral(ctx, "Should be called in the voice channel your are in.").await?;
        return Ok(());
    }

    let manager = Arc::clone(&get_songbird(ctx.serenity_context()).await.unwrap());
    let _ = manager.join(guild.id, connect_to).await;

    poise::send_reply(ctx, |builder| {
        builder
            .embed(|embed| {
                embed
                    .title(format!("Joined {}", Mention::Channel(connect_to)))
                    .color(APPLICATION_COLOUR)
                    .timestamp(Local::now())
            })
    }).await?;

    Ok(())
}


#[poise::command(slash_command, guild_only)]
pub async fn disconnect(ctx: Context<'_>) -> Result<(), ApplicationError> {
    let guild = ctx.guild().unwrap();

    let connect_to = 
        match guild.voice_states.get(&ctx.author().id).and_then(|s| s.channel_id) {
            Some(channel_id) => channel_id,
            None => {
                reply_ephemeral(ctx, "Should be called in the voice channel your are in.").await?;
                return Ok(());
            }
        };

    if connect_to != ctx.channel_id() {
        reply_ephemeral(ctx, "Should be called in the voice channel your are in.").await?;
        return Ok(());
    }

    let manager = Arc::clone(&get_songbird(ctx.serenity_context()).await.unwrap());
    if let Some(_) = manager.get(guild.id) {
        manager.remove(guild.id).await?;

        poise::send_reply(ctx, |builder| {
            builder
                .embed(|embed| {
                    embed
                        .title("Disconnected.")
                        .color(APPLICATION_COLOUR)
                        .timestamp(Local::now())
                })
        }).await?;
    } else {
        reply_ephemeral(ctx, "Not in a voice channel.").await?;
    }
    Ok(())
}


#[poise::command(slash_command, guild_only)]
pub async fn ignore(ctx: Context<'_>) -> Result<(), ApplicationError> {
    let sender = ctx.author().id;

    let mut data_lock = ctx.data().voicevox.lock().await;
    let _ = data_lock.ignored_users.insert(sender.clone()) || data_lock.ignored_users.remove(&sender);
    
    reply_ephemeral(ctx, format!(
        "Set ignored to {}.", data_lock.ignored_users.contains(&sender)
    )).await?;
    Ok(())
}
