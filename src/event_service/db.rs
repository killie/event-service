#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

const DEFAULT_DB_URL: &'static str = "postgres://postgres@localhost:5432";

pub fn connect_to_db() -> Option<PgConnection> {
    let db_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DB_URL));
    match PgConnection::establish(&db_url) {
        Ok(connection) => Some(connection),
        Err(error) => {
            println!("Could not connect to database: {}", error.description());
            None
        }
    }
}

pub fn insert_comment(
    new_comment: NewComment,
    db_connection: &PgConnection,
) -> FutureResult<i64, hyper::Error> {
    use schema::comments;
    let id = diesel::insert_into(comments::table)
        .values(&new_comment)
        .returning(comments::id)
        .get_result(db_connection);

    match id {
        Ok(id) => futures::future::ok(id),
        Err(error) => {
            println!("Error writing to database: {}", error.description());
        }
    }
}
