use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Ask {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct Reply {
    pub answer: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn ask(Json(req): Json<Ask>) -> Json<Reply> {
    let prompt = req.prompt.trim();

    if prompt.is_empty() {
        return Json(Reply {
            answer: "Prompt cannot be empty.".to_string(),
        });
    }

    if prompt.len() > 10000 {
        return Json(Reply {
            answer: "Prompt exceeds maximum allowed size.".to_string(),
        });
    }

    let answer = esprit_rag::ask(prompt).unwrap_or_else(|_| "AI request failed.".to_string());

    Json(Reply { answer })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ask", post(ask))
}
