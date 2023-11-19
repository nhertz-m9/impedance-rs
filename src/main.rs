mod commands;
mod helpers;
mod voicevox;
mod debug;

use std::{
    env, 
    collections::HashSet
};
use poise::{
    serenity_prelude::{
        GatewayIntents, 
        GuildId, 
        Activity, 
        UserId,
        Colour
    },
    Event
};
use songbird::SerenityInit;
use commands::{
    connect,
    disconnect
};
use debug::debug;
use helpers::{
    ApplicationError,
    speak
};
pub const APPLICATION_COLOUR: Colour = Colour(3908956);


async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, (), ApplicationError>
) -> Result<(), ApplicationError> {
    match event {
        Event::Ready { data_about_bot } => {
            let user = &data_about_bot.user;
            println!("Bot: {} is all set.", user.tag());

            ctx.set_activity(Activity::listening("/connect")).await;
        }
        Event::Message { new_message } => {
            speak(ctx, new_message).await?;
        }
        _ => {}
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    voicevox::version().await.expect("Try again with VOICEVOX.");
    
    dotenv::dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN should be set.");
    let guild_id = GuildId(
        env::var("GUILD").expect("GUILD should be set.").parse::<u64>().unwrap());

    let owners = match env::var("OWNER") {
        Ok(user_id) => {
            HashSet::<UserId>::from_iter([
                user_id.parse().unwrap()
            ])
        }
        Err(_) => HashSet::new()
    };

    let intents = GatewayIntents::non_privileged() 
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            owners,
            commands: vec![
                debug(),
                connect(),
                disconnect()
            ],
            event_handler: |ctx, event, framework, _| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
        .token(&token)
        .intents(intents)
        .setup(move |ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild_id).await?;
                Ok(())
            })
        })
        .client_settings(|client| {
            client.register_songbird()
        });
   
    println!("Token: {}", token);
    println!("Guild: {}", guild_id);

    framework.run().await.unwrap();
}
