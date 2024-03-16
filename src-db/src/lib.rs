pub mod create;
pub mod list;
pub mod model;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::Mutex;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

lazy_static! {
    static ref CONNECTION_URL: Mutex<String> = Mutex::new(String::new());
}

pub fn establish_connection() -> SqliteConnection {
    let connection_url = CONNECTION_URL.lock().unwrap();
    SqliteConnection::establish(&*connection_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", *connection_url))
}

pub fn setup_db(connection_url: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("running migrations");
    let mut locked_url = CONNECTION_URL.lock().unwrap();
    *locked_url = connection_url.to_owned();
    drop(locked_url); // Release the lock before establishing the connection
    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    Ok(())
}
