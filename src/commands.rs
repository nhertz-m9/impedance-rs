use std::sync::Arc;
use chrono::Local;
use poise::serenity_prelude::Mention;
use crate::{
    APPLICATION_COLOUR,
    helpers::{
        Context,
        ApplicationError,
        reply_ephemeral
    }
};


#[poise::command(slash_command)]
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

    let manager = Arc::clone(&songbird::get(ctx.serenity_context()).await.unwrap());
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


#[poise::command(slash_command)]
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

    let manager = Arc::clone(&songbird::get(ctx.serenity_context()).await.unwrap());
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
