mod commands;
mod helpers;
mod voicevox;
mod debug;
mod event_handler;
mod application_state;

use std::{
    env, 
    collections::HashSet
};
use poise::serenity_prelude::{
    GatewayIntents, 
    GuildId, 
    UserId,
    Colour
};
use songbird::SerenityInit;
use commands::{
    connect,
    disconnect
};
use debug::debug;
use helpers::ApplicationError;
use event_handler::event_handler;

pub const APPLICATION_COLOUR: Colour = Colour(3908956);


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

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
        .token(
            env::var("TOKEN").expect("TOKEN should be set.")
        )
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
        .setup(move |ctx, _, framework| {
            Box::pin(async move {
                let guild_id = env::var("GUILD")
                    .expect("GUILD should be set.").parse::<GuildId>()?;
                
                poise::builtins::register_in_guild(&ctx.http, &framework.options().commands, guild_id).await?;

                let state = application_state::ApplicationState {
                    voicevox: voicevox::VoicevoxClient::new("http://localhost:50021/")
                };
                application_state::init(ctx, state).await;
                Ok(())
            })
        })
        .intents(intents)
        .client_settings(|client| {
            client.register_songbird()
        });
   
   
    framework
        .run()
        .await
        .unwrap();
}
