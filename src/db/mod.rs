use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

//#[path = "dto.rs"]
pub mod dto;
//#[path = "models.rs"]
mod models;

pub mod events;
pub mod comments;

const DEFAULT_DB_URL: &'static str = "postgres://postgres@localhost:5432";

pub fn connect_to_db() -> Option<PgConnection> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DB_URL));
    match PgConnection::establish(&db_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            println!("Could not connect to database: {:?}", error);
            None
        }
    }
}
