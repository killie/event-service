pub use super::dto;

type EventId = i64;
type CommentId = i64;

pub fn get_comments(id: EventId) {
    println!("Loading comments for event {}", id);
}

pub fn add_comment(comment: dto::NewComment) {
    println!("Adding comment {:?}", comment);
}

pub fn delete_comment(id: CommentId) {
    println!("Deleting comment {}", id);
}

pub fn edit_comment(id: CommentId, text: String) {
    println!("Editing comment {} = {}", id, text);
}

