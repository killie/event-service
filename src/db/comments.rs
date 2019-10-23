use postgres::{Connection, error::Error};

use crate::rest::dto;

type EventId = i32;
type CommentId = i32;

pub fn get_comments(event_id: EventId, conn: &Connection) -> Result<Vec<dto::Comment>, Error> {

    let query = "SELECT id, event_id, username, message, timestamp
                 FROM comments
                 WHERE event_id = $1
                 ORDER BY timestamp;";

    let statement = match conn.prepare(query) {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    let result = statement.query(&[&event_id]);
    match result {
        Ok(rows) => {
            let mut comments: Vec<dto::Comment> = vec![];
            for row in rows.iter() {
                comments.push(dto::Comment {
                    id: row.get(0),
                    event_id: row.get(1),
                    user: row.get(2),
                    text: row.get(3),
                    timestamp: row.get(4),
                });
            }
            Ok(comments)
        },
        Err(e) => Err(e),
    }
}

pub fn add_comment(c: dto::NewComment, conn: &Connection) -> Result<CommentId, Error> {

    let query = "INSERT INTO comments (event_id, username, message, timestamp)
                 VALUES ($1, $2, $3, $4)
                 RETURNING id;";
    
    let statement = match conn.prepare(query) {
        Ok(statement) => statement,
        Err(e) => return Err(e),
    };

    let result = statement.query(&[&c.event_id, &c.user, &c.text, &c.timestamp]);
    match result {
        Ok(rows) => {
            let mut id: i32 = 0;
            for row in rows.iter() {
                id = row.get("id");
            }
            Ok(id)
        },
        Err(e) => Err(e),
    }
}

pub fn delete_comment(id: CommentId, conn: &Connection) -> Result<u64, Error> {
    conn.execute("DELETE FROM comments WHERE id = $1;", &[&id])
}

pub fn edit_comment(id: CommentId, text: String, conn: &Connection) -> Result<u64, Error> {
    conn.execute("UPDATE comments SET message = $1 WHERE id = $2", &[&text, &id])
}

