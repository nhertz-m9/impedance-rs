use std::sync::Arc;

use poise::ReplyHandle;
use songbird::{
    driver::Bitrate::BitsPerSecond,
    input::{
        ffmpeg,
        cached::Compressed
    }
};
use crate::application_state::ApplicationState;

pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), ApplicationError>;


pub async fn reply_ephemeral(
    ctx: Context<'_>, 
    content: impl Into<String>
) -> Result<ReplyHandle<'_>, poise::serenity_prelude::Error> {
    poise::send_reply(ctx, |builder| {
        builder
            .ephemeral(true)
            .content(content)
    }).await
}


pub async fn speak(
    ctx: &poise::serenity_prelude::Context, 
    message: &poise::serenity_prelude::Message
) -> Result<(), ApplicationError> {
    if message.author.bot {
        return Ok(());
    }
    
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler) = manager.get(message.guild_id.unwrap()) {
        if handler.lock().await.current_channel().unwrap().0 == message.channel_id.0 {
            let data_clone = Arc::clone(&ctx.data);
            let data_read  = data_clone.read().await
                .get::<ApplicationState>()
                .unwrap()
                .clone();

            data_read.voicevox.synthesis(message.content.clone(), 8).await?;
            let src = Compressed::new(
                ffmpeg("out.wav").await?,
                BitsPerSecond(128_000)
            ).unwrap();
            src.raw.spawn_loader();
            
            let _ = handler.lock().await.enqueue_source(src.into());
        }
    }
    
    Ok(())
}
