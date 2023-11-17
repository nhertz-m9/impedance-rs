mod commands;
use std::env;
use poise::serenity_prelude::{
    GatewayIntents, 
    GuildId, 
    Activity
};
use commands::ping;

#[derive(thiserror::Error, Debug)]
pub enum ApplicationError {
    #[error("{0}")]
    Serenity(#[from] poise::serenity_prelude::Error)
}

pub type Context<'a> = poise::Context<'a, (), ApplicationError>;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN");
    let guild_id = env::var("GUILD").expect("GUILD").parse::<GuildId>().unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions { 
            commands: vec![
                ping()
            ],
            ..Default::default()
        })
         .token(&token)
         .intents(GatewayIntents::non_privileged())
         .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;

                let user = ctx.http.get_current_user().await?;
                println!("Bot: {} is all set.", user.tag());
                
                ctx.set_activity(Activity::listening("/play")).await;
                Ok(())
            })
        });
   
    println!("Token: {}", token);
    println!("Guild: {}", guild_id);

    framework.run().await.unwrap()
}
