extern crate todos_db;
mod common;

use common::establish_test_connection;
use serde_json::Value;
use todos_db::create::todos::create_todo;
use todos_db::list::todos::list_todos;

#[test]
fn test_adding_todo() {
    let mut conn = establish_test_connection();
    let todo_name = "New todo";
    create_todo(&mut conn, todo_name);

    let todos_json = list_todos(&mut conn);
    let todos: Value = serde_json::from_str(&todos_json).expect("Failed to parse JSON");

    // Assuming `todos` is an array of todo items
    let todo_added = todos
        .as_array()
        .unwrap()
        .iter()
        .any(|todo| todo.get("name").and_then(|name| name.as_str()) == Some(todo_name));

    assert!(todo_added, "Todo was not added successfully");
}
