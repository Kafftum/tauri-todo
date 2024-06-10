const invoke = window.__TAURI__.invoke;

let todo_input = $("#todo_input");
let add_button = $("#add_button");
let todo_display = $("#todo_display");

await invoke("return_vec").then((todo_list) => {
  let i;
  for(i = 0; i < todo_list.length; i++) {
    createTodo(todo_list[i]);
  }
})

add_button.click(async () => {
  await invoke("store_in_vec", { todo: todo_input.val() });
  await invoke("return_vec").then((todo_list) => {
    todo_display.empty();
    
    let i;
    for(i = 0; i < todo_list.length; i++) {
      createTodo(todo_list[i]);
    }
  });
});

function createTodo(title) {
  let todo = $("<div>").html(`<h3 class="todo_text">${title}</h3>
                              <button class="remove_todo">
                                Remove
                              </button>`);

  todo.addClass("todo_block");
  todo.find(".remove_todo").click(async (e) => {
    e.preventDefault();
    await invoke("remove_todo", { todo: title });
    todo.remove();
  });
  todo_display.append(todo);
}