use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

pub fn embed(model: &str, text: &str) -> Result<Vec<f32>> {
    let res: EmbedResponse = Client::new()
        .post("http://127.0.0.1:11434/api/embed")
        .json(&EmbedRequest { model, input: text })
        .send()?
        .json()?;

    Ok(res.embeddings.into_iter().next().unwrap_or_default())
}
