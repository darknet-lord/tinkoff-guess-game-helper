/*
 *  $ cargo run --bin web_app
 *
 *  $ curl -H "Content-Type: application/json" -X POST http://localhost:5000/guess-word \
 *  -d '["yяgмgнyдyа"]'
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
    "Usage: POST /guest-word/ yяgмgнyдyа"
}

async fn guess_word_api(Json(payload): Json<Vec<String>>) -> impl IntoResponse {
    let words = strings_to_words(payload);
    let guesses = vec![guess_word(words)];
    (StatusCode::OK, Json(guesses))
}

#[derive(Deserialize)]
struct GuessWord {
    letters: Vec<String>,
}

#[derive(Serialize)]
struct Word {
    letters: String,
}
