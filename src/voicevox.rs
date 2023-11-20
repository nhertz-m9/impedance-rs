use reqwest::{
    Client,
    header
};
use crate::ApplicationError;

pub struct VoicevoxClient {
    client: Client,
    base_url: String
}

impl VoicevoxClient {

    pub fn new(base_url: impl Into<String>) -> Self {
        Self { 
            client: Client::new(), 
            base_url: base_url.into()
        }
    }

    pub async fn synthesis(&self, content: impl Into<String>, speaker: u8) -> Result<(), ApplicationError> {

        let query = self.client
            .post(self.base_url.clone() + "audio_query")
            .header(header::ACCEPT, "application/json")
            .query(&[("speaker", &speaker.to_string()), ("text", &content.into())])
            .send()
            .await?
            .text()
            .await?;
    
        let res = self.client
            .post(self.base_url.clone() + "synthesis")
            .header(header::CONTENT_TYPE, "application/json")
            .query(&[("speaker", &speaker.to_string())])
            .body(query)
            .send()
            .await?
            .bytes()
            .await?;
    
        
        std::fs::write("out.wav", res)?;
        Ok(())
    }
}
