use std::sync::Arc;
use tokio::sync::Mutex;

use crate::voicevox::VoicevoxClient;

pub struct ApplicationState {
    pub voicevox: Arc<Mutex<crate::voicevox::VoicevoxClient>>
}

impl ApplicationState {
    pub fn new() -> Self {
        Self {
            voicevox: Arc::new(Mutex::new(
                VoicevoxClient::new("http://localhost:50021/")
            ))
        }
    }
}
