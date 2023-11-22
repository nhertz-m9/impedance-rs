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

pub async fn init_application_state(ctx: &Context, state: ApplicationState) {
    let data = Arc::clone(&ctx.data);
    let mut data_write = data.write().await;

    data_write.insert::<ApplicationState>(Arc::new(
        state
    ));
}
