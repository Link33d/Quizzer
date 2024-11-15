// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod models;
mod utils;

#[tauri::command]
fn get_random_question() -> models::QuestionResponse {
    return commands::get_random_question();
}

#[tauri::command]
fn check_question_answer(data: models::Answer) -> bool {
    return commands::check_question_answer(data);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_random_question, check_question_answer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
