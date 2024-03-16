use diesel::prelude::*;

use crate::model::*;
use crate::schema::*;

pub fn create_todo(conn: &mut SqliteConnection, name: &str) -> Todo {
    let new_todo = NewTodo { name };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new todo")
}
