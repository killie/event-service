pub use super::dto;

pub fn add_comment(comment: dto::Comment) {
    println!("Adding comment {:?}", comment);
}

pub fn delete_comment() {
    println!("Deleting comment");
}

pub fn edit_comment() {
    println!("Editing comment");
}

