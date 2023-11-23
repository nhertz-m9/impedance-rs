use std::sync::Arc;

use songbird::{
    driver::Bitrate::BitsPerSecond, 
    input::cached::Compressed, 
    ffmpeg 
};
use crate::{
    application_state::ApplicationState, 
    helpers::{
        ApplicationError, 
        get_songbird
    }, 
    constants::DEFAULT_SPEAKER_ID
};


pub async fn speak(
    ctx: &poise::serenity_prelude::Context, 
    message: &poise::serenity_prelude::Message,
    data: &ApplicationState
) -> Result<(), ApplicationError> {
    let client_lock = data
        .voicevox
        .lock()
        .await;
    
    if message.author.bot || client_lock.ignored_users.contains(&message.author.id) {
        return Ok(());
    }

    let manager = Arc::clone(&get_songbird(ctx).await?);

    if let Some(call) = manager.get(message.guild_id.unwrap()) {
        let connect_to = call
            .lock()
            .await
            .current_channel()
            .unwrap();

        if connect_to == message.channel_id.into() {
           
            client_lock.synthesis(
                message.content.clone(), 
                match client_lock.voice.get(&message.author.id) {
                    Some(id) => *id,
                    _ => DEFAULT_SPEAKER_ID
                }
            ).await?;

            let src = Compressed::new(
                ffmpeg("out.wav").await?,
                BitsPerSecond(128_000)
            ).unwrap();
            src.raw.spawn_loader();
            
            let _ = call.lock().await.enqueue_source(src.into());
        }
    }
    
    Ok(())
}
