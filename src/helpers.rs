use std::sync::Arc;

use poise::ReplyHandle;
use songbird::Songbird;
use crate::application_state::ApplicationState;

pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, ApplicationState, ApplicationError>;


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


pub async fn get_songbird(
    ctx: &poise::serenity_prelude::Context
) -> Result<Arc<Songbird>, ApplicationError> {

    Ok(songbird::get(ctx)
        .await
        .expect("songbird not intialized"))
}
