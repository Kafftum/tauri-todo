use std::sync::Mutex;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct MW {
  todos: Mutex<Vec<String>>
}

fn main() {
    tauri::Builder::default()
      .manage(MW{ todos: Mutex::new(vec![])})
      .invoke_handler(tauri::generate_handler![store_in_vec, return_vec, remove_todo])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn store_in_vec(state: tauri::State<MW>, todo: String) {
  state.todos.lock().unwrap().push(todo);
}

#[tauri::command]
fn return_vec(state: tauri::State<MW>) -> Vec<String> {
  state.todos.lock().unwrap().clone()
}

#[tauri::command]
fn remove_todo(state: tauri::State<MW>, todo: String) {
  let mut todos = state.todos.lock().unwrap();
  let mut i: usize = 0;
  loop {
    if i > todos.len() {
      break;
    }

    if todos[i] == todo {
      todos.remove(i);
      break;
    }

    i += 1;
  }
}