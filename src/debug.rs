use crate::helpers::{
    ApplicationError,
    Context
};

#[poise::command(slash_command, guild_only, owners_only)]
pub async fn debug(
    _ctx: Context<'_>,
) -> Result<(), ApplicationError> {
   
    Ok(())    
}
