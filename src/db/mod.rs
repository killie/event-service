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
    let connection = connect_to_db()?;
    // Create tables
    Ok(())
}
