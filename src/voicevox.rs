use std::collections::{
    HashSet, 
    HashMap
};
use serde::Deserialize;
use poise::serenity_prelude::UserId;
use reqwest::{
    Client,
    header
};
use crate::ApplicationError;

pub struct VoicevoxClient {
    client: Client,
    base_url: String,
    pub ignored_users: HashSet<UserId>,
    pub voice: HashMap<UserId, u8>
}

impl VoicevoxClient {

    pub fn new(base_url: impl Into<String>) -> Self {
        Self { 
            client: Client::new(),
            base_url: base_url.into(),
            ignored_users: HashSet::new(),
            voice: HashMap::new()
        }
    }

    pub async fn synthesis(&self, content: impl Into<String>, speaker: u8) -> Result<(), ApplicationError> {

        let query = self.client
            .post(
                self.get_endpoint("audio_query")
            )
            .header(header::ACCEPT, "application/json")
            .query(&[
                ("speaker", speaker.to_string()), 
                ("text", content.into())
            ])
            .send()
            .await?
            .text()
            .await?;
    
        let res = self.client
            .post(
                self.get_endpoint("synthesis")
            )
            .header(header::CONTENT_TYPE, "application/json")
            .query(&[
                ("speaker", speaker.to_string())
            ])
            .body(query)
            .send()
            .await?
            .bytes()
            .await?;
    
        
        std::fs::write("out.wav", res)?;
        Ok(())
    }

    pub async fn get_speakers(&self) -> Result<Vec<Speaker>, ApplicationError> {
        let res = self.client
            .get(
                self.get_endpoint("speakers")
            )
            .send()
            .await?
            .json::<Vec::<Speaker>>()
            .await?;

        Ok(res)
    }

    pub fn get_endpoint(&self, path: impl AsRef<str>) -> String {
        self.base_url.clone() + path.as_ref()
    }
}


#[derive(Clone, Deserialize)]
pub struct Speaker {
    pub name: String,
    pub styles: Vec<Style>
}

#[derive(Clone, Deserialize)]
pub struct Style {
    pub name: String,
    pub id: u32
}
