use std::sync::Arc;

use poise::{
    serenity_prelude::Activity, 
    Event
};
use crate::{
    ApplicationError, 
    helpers::speak,
    voice_state::get_vc_users
};

pub async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, (), ApplicationError>
) -> Result<(), ApplicationError> {
    match event {

        Event::Ready { data_about_bot } => {
            let user = &data_about_bot.user;
            println!("{} is all set.", user.tag());

            ctx.set_activity(Activity::listening("/connect")).await;
        }

        Event::Message { new_message } => {
            speak(ctx, new_message)
                .await.unwrap_or_else(|_| println!("unable to connect to voicevox."));
        }

        Event::VoiceStateUpdate { old, new } => {
            if old.is_some() && new.channel_id.is_none() {
                
                let guild = old.clone().unwrap().guild_id.unwrap();
                let channel = old.clone().unwrap().channel_id.unwrap();
               
                if get_vc_users(ctx, guild, channel)?.len() == 1 {
                    let manager = Arc::clone(&songbird::get(ctx).await.unwrap());
                    manager.remove(guild).await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
