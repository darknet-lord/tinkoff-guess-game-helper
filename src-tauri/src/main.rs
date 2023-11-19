// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tinkoff_guess_game_lib::{guess_word, strings_to_words};

#[tauri::command]
fn get_suggestions(words: Vec<String>) -> Vec<&'static str> {
  guess_word(strings_to_words(words))
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_suggestions])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
