use poise::{
    serenity_prelude::Activity, 
    Event
};
use crate::{
    ApplicationError, 
    helpers::speak
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
        _ => {}
    }
    Ok(())
}
