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
    endpoint: String,
}

impl Ai {
    pub fn new(model: impl Into<String>) -> anyhow::Result<Self> {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .map_err(|error| anyhow::anyhow!("failed to create HTTP client: {error}"))?,
            model: model.into(),
            endpoint: std::env::var("OLLAMA_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string()),
        })
    }

    pub fn health(&self) -> Result<()> {
        self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .map_err(|_| anyhow!("Ollama is not running"))?;
        Ok(())
    }

    pub fn ask(&self, prompt: &str) -> Result<String> {
        let res: GenerateResponse = self
            .client
            .post(format!("{}/api/generate", self.endpoint))
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
