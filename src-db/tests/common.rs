use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_test_connection() -> SqliteConnection {
    let mut connection = SqliteConnection::establish(":memory:")
        .expect("Failed to establish a connection to an in-memory SQLite database");

    // Enable foreign key support for SQLite, as it's not enabled by default
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut connection)
        .expect("Failed to enable foreign key support");

    // Apply migrations to the in-memory database
    connection.run_pending_migrations(MIGRATIONS).unwrap();

    connection
}
