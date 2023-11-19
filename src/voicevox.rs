use reqwest::Client;
use crate::ApplicationError;

pub async fn version() -> Result<String, ApplicationError> {
    let res = Client::new()
        .get("http://localhost:50021/version")
        .send()
        .await?;

    Ok(res.text().await?)
}


pub async fn synthesis(content: impl Into<String>, speaker: u8) -> Result<(), ApplicationError> {

    let url = "http://localhost:50021/".to_string();

    let query = Client::new()
        .post(url.clone() + "audio_query")
        .header(reqwest::header::ACCEPT, "application/json")
        .query(&[("text", &content.into()), ("speaker", &speaker.to_string())])
        .send()
        .await?
        .text()
        .await?;

    let res = Client::new()
        .post(url.clone() + "synthesis")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .query(&[("speaker", &speaker.to_string())])
        .body(query)
        .send()
        .await?
        .bytes()
        .await?;

    
    std::fs::write("out.wav", res)?;
    Ok(())
}
