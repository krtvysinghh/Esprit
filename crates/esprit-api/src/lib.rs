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
    let answer = esprit_rag::ask(&req.prompt).unwrap_or_else(|e| e.to_string());

    Json(Reply { answer })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ask", post(ask))
}
