use diesel::prelude::*;

use crate::model::*;
use crate::schema::*;

pub fn list_todos(conn: &mut SqliteConnection) -> String {
    let results = todos::table
        .select(Todo::as_select())
        .load::<Todo>(conn)
        .expect("Error loading todos");

    serde_json::to_string_pretty(&results).expect("Error converting to JSON")
}
