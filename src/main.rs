mod commands;
mod helpers;
mod voicevox;
mod debug;
mod event_handler;
mod application_state;
mod voice_state;
mod speak;
mod constants;

use std::{
    env, 
    collections::HashSet
};
use poise::{
    builtins::register_in_guild,
    serenity_prelude::{
        GatewayIntents, 
        GuildId, 
        UserId
    }
};
use songbird::SerenityInit;
use commands::{
    connect,
    disconnect,
    ignore
};
use debug::debug;
use helpers::ApplicationError;
use event_handler::event_handler;
use application_state::ApplicationState;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let owners = match env::var("OWNER") {
        Ok(user_id) => {
            HashSet::<UserId>::from_iter([
                user_id.parse().unwrap()
            ])
        }
        _ => HashSet::new()
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
                disconnect(),
                ignore()
            ],
            event_handler: |ctx, event, _, data| {
                Box::pin(event_handler(ctx, event, data))
            },
            ..Default::default()
        })  
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                let guild_id = env::var("GUILD")
                    .expect("GUILD should be set.").parse::<GuildId>()?;
                register_in_guild(&ctx.http, &framework.options().commands, guild_id).await?;

                Ok(ApplicationState::new())
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
