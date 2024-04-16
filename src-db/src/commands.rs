use tauri;

use crate::establish_connection;
use crate::create::todos::create_todo as db_create_todos;
use crate::list::todos::list_todos as db_list_todos;

#[tauri::command(async, rename_all = "snake_case")]
pub fn add_todo(todo: String) -> i32 {
    let conn = &mut establish_connection();
    let todo = db_create_todos(conn, &todo);
    todo.id
}

#[tauri::command(async, rename_all = "snake_case")]
pub fn list_todos() -> String {
    let conn = &mut establish_connection();
    db_list_todos(conn)
}
