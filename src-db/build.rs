const COMMANDS: &[&str] =
    &["list_todos", "add_todo"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
