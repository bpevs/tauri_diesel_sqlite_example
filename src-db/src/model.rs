use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = todos)]
pub struct Todo {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub name: &'a str,
}
