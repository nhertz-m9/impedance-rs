use poise::serenity_prelude::{
    Context, 
    GuildId, 
    ChannelId, 
    UserId
};
use crate::helpers::ApplicationError;


pub fn get_vc_users(
    ctx: &Context,
    guild: GuildId,
    channel: ChannelId
) -> Result<Vec<UserId>, ApplicationError> {

    let voice_states = guild
        .to_guild_cached(&ctx.cache)
        .unwrap()
        .voice_states;

    let list = voice_states
        .into_iter()
        .filter(|(_, state)| state.channel_id == Some(channel))
        .map(|(_, state)| state.user_id)
        .collect();

    Ok(list)
}
