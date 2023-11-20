use std::sync::Arc;
use poise::serenity_prelude::{
    TypeMapKey, 
    Context
};

pub struct ApplicationState {
    pub voicevox: crate::voicevox::VoicevoxClient
}

impl TypeMapKey for ApplicationState {
    type Value = Arc<ApplicationState>;
}

pub async fn init(ctx: &Context, state: ApplicationState) {
    let mut data = ctx.data.write().await;

    data.insert::<ApplicationState>(Arc::new(
        state
    ));
}
