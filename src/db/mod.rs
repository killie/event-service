use dotenv::dotenv;
use std::env;
use postgres::{Connection, TlsMode, Error};

pub mod events;
pub mod comments;

const DEFAULT_DB_URL: &'static str = "postgres://postgres@localhost:5432";

pub fn connect_to_db() -> Result<Connection, Error> {
    dotenv().ok(); // TODO: What is this then
    let db_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DB_URL));
    let result = Connection::connect(db_url, TlsMode::None);
    result
}

pub fn run_updates() -> Result<(), Error> {
    let conn = connect_to_db()?;
    create_events_table(&conn)?;
    create_comments_table(&conn)?;
    Ok(())
}

fn create_events_table(conn: &Connection) -> Result<u64, Error> {
    let command = "CREATE TABLE IF NOT EXISTS events (
                   id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                   \"from\" BIGINT,
                   \"to\" BIGINT,
                   origin_id INTEGER,
                   event_type INTEGER,
                   message TEXT);";

    conn.execute(command, &[])
}

fn create_comments_table(conn: &Connection) -> Result<u64, Error> {
    let command = "CREATE TABLE IF NOT EXISTS comments (
                   id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                   event_id INTEGER,
                   username TEXT,
                   message TEXT,
                   timestamp BIGINT);";

    conn.execute(command, &[])
}
