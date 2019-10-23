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
                Ok(connection) => {
                    match db::comments::add_comment(comment, &connection) {
                        Ok(id) => {
                            println!("id: {}", id);
                            super::id_response(id)
                        },
                        Err(error) => {
                            println!("Could not add record to database: {}", error);
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

pub fn get_comments_by_event_id(path: &str) -> ResponseFuture {
    match super::get_id_from_path(&path, "/comments/") {
        Some(event_id) => {
            match db::connect_to_db() {
                Ok(connection) => {
                    match db::comments::get_comments(event_id, &connection) {
                        Ok(comments) => {
                            let comments = serde_json::to_string(&comments).unwrap();
                            let envelope = envelope::success_from_str(comments);
                            super::send_result(&envelope)
                        },
                        Err(error) => {
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
        },
        None => {
            super::empty_response(StatusCode::BAD_REQUEST)
        },
    }
}

pub fn delete_comment_by_id(path: &str) -> ResponseFuture {
    match super::get_id_from_path(&path, "/comments/") {
        Some(comment_id) => {
            match db::connect_to_db() {
                Ok(connection) => {
                    match db::comments::delete_comment(comment_id, &connection) {
                        Ok(count) => {
                            if count == 1 {
                                super::empty_response(StatusCode::OK)
                            } else {
                                super::empty_response(StatusCode::NOT_FOUND)
                            }
                        },
                        Err(error) => {
                            println!("Error deleting comment id {} ({})", comment_id, error);
                            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                        },
                    }
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        None => {
            super::empty_response(StatusCode::BAD_REQUEST)
        },
    }
}
