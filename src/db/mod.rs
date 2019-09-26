use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use futures::future::FutureResult;

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

pub fn insert_comment(
    comment: dto::Comment,
    db_connection: &PgConnection,
    //) -> FutureResult<i64, hyper::Error> {
    ) {
    /*
    use schema::comments;
    
    let new_comment = models::NewComment {
        event_id: comment.event_id,
        username: comment.user,
        message: comment.text,
    };

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
     */

}
