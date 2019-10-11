use diesel::pg::{PgConnection};
use diesel::{deserialize::FromSql};
use diesel::result::{Error as DieselError, QueryResult};
use crate::diesel::RunQueryDsl;

pub use super::dto; // TODO: Remove from db mod into root
use super::models;
use crate::schema::comments;

type EventId = i32;
type CommentId = i32;

pub fn get_comments(id: EventId) {
    //let result: i32 = comments::table.load::<models::Comment>(&connection).get_result(connection);
    println!("Loading comments for event {}", id);
}

pub fn add_comment(comment: dto::NewComment, connection: &PgConnection) -> CommentId {
    let new_comment = models::NewComment {
        event_id: comment.event_id,
        username: comment.user,
        message: comment.text,
        timestamp: comment.timestamp,
    };

    diesel::insert_into(comments::table)
        .values(&new_comment)
        .returning(comments::id)
        .get_result(connection)
        .expect("Error saving new comment.")
}

pub fn delete_comment(id: CommentId) {
    println!("Deleting comment {}", id);
}

pub fn edit_comment(id: CommentId, text: String) {
    println!("Editing comment {} = {}", id, text);
}

