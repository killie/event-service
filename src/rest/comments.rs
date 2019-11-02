extern crate hyper;

use hyper::StatusCode;
use serde_json::Result;

use crate::db;
use crate::rest::{ResponseFuture, dto, envelope};

pub fn add_comment(chunk: hyper::Chunk) -> ResponseFuture {
    // Read entire body from request as string
    let str_body = String::from_utf8(chunk.to_vec()).unwrap();
    // Deserialize JSON string to NewComment
    let parse_result: Result<dto::NewComment> = serde_json::from_str(&str_body);
    match parse_result {
        Ok(comment) => {
            match db::connect_to_db() {
                Ok(conn) => {
                    match db::comments::add_comment(comment, &conn) {
                        Ok(id) => {
                            println!("id: {}", id);
                            super::id_response(id)
                        },
                        Err(e) => {
                            println!("Could not add record to database: {}", e);
                            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                        }}
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Invalid body: {}", str_body);
            super::empty_response(StatusCode::BAD_REQUEST)
        },
    }
}

pub fn get_comments_by_event_id(event_id: i32) -> ResponseFuture {
    match db::connect_to_db() {
        Ok(conn) => {
            match db::comments::get_comments(event_id, &conn) {
                Ok(comments) => {
                    let comments = serde_json::to_string(&comments).unwrap();
                    let envelope = envelope::success_from_str(comments);
                    super::send_result(&envelope)
                },
                Err(_) => {
                    println!("Error loading comments");
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Could not connect to database.");
            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub fn delete_comment_by_id(comment_id: i32) -> ResponseFuture {
    match db::connect_to_db() {
        Ok(conn) => {
            match db::comments::delete_comment(comment_id, &conn) {
                Ok(count) => {
                    if count == 1 {
                        println!("Comment {} deleted.", comment_id);
                        super::empty_response(StatusCode::OK)
                    } else {
                        super::empty_response(StatusCode::NOT_FOUND)
                    }
                },
                Err(e) => {
                    println!("Error deleting comment id {} ({})", comment_id, e);
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Could not connect to database.");
            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub fn edit_comment(chunk: hyper::Chunk) -> ResponseFuture {
    let str_body = String::from_utf8(chunk.to_vec()).unwrap();
    let parse_result: Result<dto::EditComment> = serde_json::from_str(&str_body);
    match parse_result {
        Ok(comment) => {
            match db::connect_to_db() {
                Ok(conn) => {
                    match db::comments::edit_comment(comment.id, comment.text, &conn) {
                        Ok(_) => {
                            println!("Comment {} updated.", comment.id);
                            super::empty_response(StatusCode::OK)
                        },
                        Err(e) => {
                            println!("Could not update record in database: {}", e);
                            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                        }}
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Invalid body: {}", str_body);
            super::empty_response(StatusCode::BAD_REQUEST)
        },
    }
}

