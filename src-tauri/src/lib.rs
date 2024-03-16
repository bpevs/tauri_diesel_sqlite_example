use src_db::*;
use std::fs::create_dir_all;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

/// Resolve the app filepath from the `AppHandle` context
fn app_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().app_config_dir().expect("No App path was found!")
}

/// Maps the user supplied DB connection string to a connection string
fn path_mapper(mut app_path: PathBuf, connection_string: &str) -> String {
    app_path.push(
        connection_string
            .split_once(':')
            .expect("Couldn't DB connection string")
            .1,
    );
    format!(
        "{}",
        app_path
            .to_str()
            .expect("Couldn't create path to Database file")
    )
}

#[tauri::command(async, rename_all = "snake_case")]
fn add_todo(todo: String) -> i32 {
    let conn = &mut establish_connection();
    let todo = src_db::create::todos::create_todo(conn, &todo);
    todo.id
}

#[tauri::command(async, rename_all = "snake_case")]
fn list_todos() -> String {
    let conn = &mut establish_connection();
    src_db::list::todos::list_todos(conn)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let db = "sqlite:my-todos.db";
            let conn_url = path_mapper(app_path(&app_handle), &db);
            create_dir_all(app_path(&app_handle)).expect("Problem creating App directory!");

            println!("{}", conn_url);
            setup_db(&conn_url).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![add_todo, list_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
