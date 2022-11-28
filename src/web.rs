/*
 *  $ cargo run --bin web_app
 *
 *  $ curl -H "Content-Type: application/json" -X POST http://localhost:5000/guess-word \
 *  -d '{"words": ["yяgмgнyдyа"]}'
 *
 *  [["ябеда","ягода"]]
 * 
 * 
 */
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tinkoff_guess_game_lib::{guess_word, strings_to_words};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/guess-word", post(guess_word_api));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Usage: curl -H \"Content-Type: application/json\" -X POST /guest-word/ -D '[\"yяgмgнyдyа\"]'"
}

async fn guess_word_api(Json(payload): Json<AttemptedWords>) -> impl IntoResponse {
    let words = strings_to_words(payload.words);
    let guesses = guess_word(words);
    (StatusCode::OK, Json(Answer{words: guesses}))
}

#[derive(Deserialize)]
struct AttemptedWords{
    words: Vec<String>,
}

#[derive(Serialize)]
struct Answer {
    words: Vec<String>,
}
