mod commands;
use std::env;
use poise::serenity_prelude::{
    GatewayIntents, 
    GuildId, 
    Activity
};
use commands::{
    ping,
    debug
};

pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), ApplicationError>;


async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, (), ApplicationError>
) -> Result<(), ApplicationError> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            let user = &data_about_bot.user;
            println!("Bot: {} is all set.", user.tag());

            ctx.set_activity(Activity::listening("/debug")).await;
        }
        _ => {}
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN should be set.");
    let guild_id = env::var("GUILD").expect("GUILD should be set.").parse::<GuildId>().unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions { 
            commands: vec![
                ping(),
                debug()
            ],
            event_handler: |ctx, event, framework, _| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
         .token(&token)
         .intents(GatewayIntents::non_privileged())
         .setup(move |ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                Ok(())
            })
        });
   
    println!("Token: {}", token);
    println!("Guild: {}", guild_id);

    framework.run().await.unwrap();
}
