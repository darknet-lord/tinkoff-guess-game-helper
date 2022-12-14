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
    response::{IntoResponse,Html},
    Json, Router,
};
use minijinja::render;

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;
use std::collections::HashMap;
use tinkoff_guess_game_lib::{guess_word, strings_to_words, suggest_words};

#[tokio::main]
async fn main() {
    // "Usage: curl -H \"Content-Type: application/json\" -X POST /guest-word/ -D '{\"words\":[\"yяgмgнyдyа\"]}'"
    let host_details = env::var("TINKOFF_GUESS_GAME_HELPER_HOST").unwrap_or("0.0.0.0:5000".to_string());
    let addr: SocketAddr = host_details.parse().expect("TINKOFF_GUESS_GAME_HELPER_HOST is incorrect");

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/guess-word", post(guess_word_api));

    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<String> {
    let r = render!(MAIN_TEMPLATE, API_HOST => env::var("TINKOFF_GUESS_GAME_HELPER_API").unwrap_or("http://localhost:5000".to_string()));
    Html(r)
}

async fn guess_word_api(Json(payload): Json<AttemptedWords>) -> impl IntoResponse {
    let guesses;
    if payload.words.len() == 0 {
        guesses = suggest_words();
    } else {
        let words = strings_to_words(payload.words);
        guesses = guess_word(words);
    }
    (StatusCode::OK, Json(Answer{words: guesses}))
}

#[derive(Deserialize)]
struct AttemptedWords{
    words: Vec<String>,
}

#[derive(Serialize)]
struct Answer<'a> {
    words: Vec<&'a str>,
}

const MAIN_TEMPLATE: &'static str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Tinkoff Guess Game Helper</title>
  <style>
      .word {margin-bottom: 4px}
      .word div {padding: 2px}
      .color_select {padding:4px}
      .bg_white {background-color: white}
      .bg_yellow {background-color: yellow}
      .bg_gray {background-color: gray}
  </style>
</head>
<body>
    <h3>{{ TINKOFF_GUESS_GAME_HELPER_API }}</h3>
    <div id="message" style="display:none; border: 1px solid gray"></div>
    {% for i in range(0, 5) %}
        <div class="word">
            <div>
                <input type="text" id="word{{ i }}"/><br />
            </div>
            <div>
                {% for j in range(0, 5) %}
                    <select id="colors-{{ i }}-{{ j }}" class="color_select bt_gray">
                        <option value="y" class="bg_yellow">y</option>
                        <option value="w" class="bg_white">w</option>
                        <option value="g" class="bg_gray" selected="selected">g</option>
                    </select>
                {% endfor %}
            </div>
        </div>
    {% endfor %}
    <button onClick="getHelp()">Get Help</button>
    <script type="text/javascript">
        const API_URL = "{{ API_URL }}";
        function colorizeWords(words) {
            var colorizedWords = [];
            for (var i = 0; i < words.length; i++) {
                var colorized = "";
                for (var j = 0; j < words[i].length; j++) {
                    var select = document.getElementById("colors-" + i + "-" + j);
                    const color = select.selectedOptions[0].value;
                    colorized += color;
                    colorized += words[i][j];
                }
                colorizedWords.push(colorized);
            }
            return colorizedWords;
        }

        function makeGuessRequest(words) {
            console.debug("Making guess-word request with: ", words);
            fetch('{{ TINKOFF_GUESS_GAME_HELPER_API }}' + '/guess-word', {
                method: 'POST',
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({"words": words})
            })
            .then(response => response.json())
            .then(response => processResponse(response));
        }

        function getHelp() {
            var words = [];
            for (var i = 0; i < 5; i++) {
                const word = document.getElementById("word" + i).value;
                if (word == "") {
                    break;
                } else {
                   words.push(word);
                }
            }
            makeGuessRequest(colorizeWords(words));
        }

        function processResponse(response) {
            console.debug("guess-word response: ", response);
            const elm = document.getElementById("message");
            if ("words" in response) {
                elm.outerHTML = response.words;
            }
            elm.style.display = 'block';
        }

        function onColorSelectChange(e) {
            const elm = e.target;
            console.log(elm);
            for (var i = 0; i < elm.options.length; i++) {
                var opt = elm.options[i];
                if (!opt.selected) elm.classList.remove(opt.className);
                else elm.classList.add(opt.className);
            }
        }
        var selects = document.getElementsByClassName("color_select");
        for (var i = 0; i < selects.length; i++) {
            selects[i].addEventListener("change", onColorSelectChange);
        };
    </script>
</body>
</html>
"#;
