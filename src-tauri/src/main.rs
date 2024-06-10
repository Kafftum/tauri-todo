use std::sync::Mutex;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct MW(Mutex<Client>);

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
  title: String
}

fn main() {
    tauri::Builder::default()
      .manage(MW(Mutex::new(Client::new())))
      .invoke_handler(tauri::generate_handler![store_in_vec, return_vec, remove_todo])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn store_in_vec(state: tauri::State<MW>, todo: String) {
  let _ = state.0.lock()
                  .unwrap()
                  .post("http://localhost:8000/add-todo")
                  .header("Content-Type", "application/json")
                  .body("{ \"title\":\"".to_owned() + &todo + "\" }")
                  .send();
}

#[tauri::command]
fn return_vec(state: tauri::State<MW>) -> Vec<String> {
  let response = state.0.lock().unwrap().get("http://localhost:8000/get-todos").send();

  serde_json::from_str::<Vec<Todo>>(&response.unwrap()
                                              .text()
                                              .unwrap()).unwrap()
                                                        .iter()
                                                        .map(|t| t.title.clone())
                                                        .collect::<Vec<String>>()
}

#[tauri::command]
fn remove_todo(state: tauri::State<MW>, todo: String) {
  let _ = state.0.lock()
                  .unwrap()
                  .post("http://localhost:8000/delete-todo")
                  .header("Content-Type", "application/json")
                  .body("{ \"title\":\"".to_owned() + &todo + "\" }")
                  .send();
}