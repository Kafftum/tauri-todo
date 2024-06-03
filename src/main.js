const invoke = window.__TAURI__.invoke;

let todo_display = document.getElementById("todo_display");

document.getElementById("todo_button").addEventListener("click", async () => {
const result = await invoke("my_first_command");

todo_display.textContent = result;
});

