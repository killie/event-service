use diesel::pg::{PgConnection};
use diesel::result::{Error as DieselError};
use crate::diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

pub use super::dto; // TODO: Remove from db mod into root
use super::models;
use crate::schema::comments;

type EventId = i32;
type CommentId = i32;

pub fn get_comments(event_id: EventId, connection: &PgConnection) -> Result<Vec<dto::Comment>, DieselError> {
    let results = comments::table
        .filter(comments::event_id.eq(&event_id))
        .load::<models::Comment>(connection);

    match results {
        Ok(data) => {
            let comments = data.iter()
                .map(|c| {
                    dto::Comment {
                        id: c.id,
                        event_id: c.event_id,
                        user: c.username.clone(),
                        text: c.message.clone(),
                        timestamp: c.timestamp,
                    }
                })
                .collect();
            return Ok(comments);
        },
        Err(error) => Err(error),
    }
}

pub fn add_comment(comment: dto::NewComment, connection: &PgConnection) -> Result<CommentId, DieselError> {
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
}

pub fn delete_comment(id: CommentId) {
    println!("Deleting comment {}", id);
}

pub fn edit_comment(id: CommentId, text: String) {
    println!("Editing comment {} = {}", id, text);
}

