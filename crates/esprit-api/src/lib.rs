use axum::{
    response::sse::{Event, Sse},
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

    let answer = match esprit_rag::ask(prompt) {
        Ok(answer) => answer,
        Err(error) => {
            tracing::error!(%error, "AI request failed");
            "AI request failed.".to_string()
        }
    };

    Json(Reply { answer })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ask", post(ask))
        .route("/ask/stream", post(stream_answer))
        .route("/ask/stream", post(stream_answer))
}

pub async fn stream_answer(
    Json(req): Json<Ask>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
    let answer = match esprit_rag::ask(req.prompt.trim()) {
        Ok(answer) => answer,
        Err(_) => "AI request failed.".to_string(),
    };

    let events = vec![Ok(Event::default().data(answer))];

    Sse::new(iter(events))
}
