use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

pub struct Ai {
    client: Client,
    model: String,
}

impl Ai {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            model: model.into(),
        }
    }

    pub fn health(&self) -> Result<()> {
        self.client
            .get(format!(
                "{}/api/tags",
                std::env::var("OLLAMA_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string())
            ))
            .send()
            .map_err(|_| anyhow!("Ollama is not running"))?;
        Ok(())
    }

    pub fn ask(&self, prompt: &str) -> Result<String> {
        let res: GenerateResponse = self
            .client
            .post(format!(
                "{}/api/generate",
                std::env::var("OLLAMA_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string())
            ))
            .json(&GenerateRequest {
                model: &self.model,
                prompt,
                stream: false,
            })
            .send()?
            .json()?;

        Ok(res.response)
    }
}
