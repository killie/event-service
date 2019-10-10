use serde::{Deserialize, Serialize};
use serde_json::{Value};

pub fn hello() {
    println!("hello says the envelope.");
}

#[derive(Serialize, Deserialize)]
pub enum EnvelopeStatus {
    OK,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub status: EnvelopeStatus,
    pub data: Option<Value>,
    pub error: Option<String>, // TODO: EnvelopeError { code: i32, description: String }
    pub page_number: Option<i32>,
    pub next_page: Option<String>,
    pub total_pages: Option<i32>
}

pub fn error(code: i32, description: String) -> Envelope {
    Envelope {
        status: EnvelopeStatus::Error,
        data: None,
        error: Some(description),
        page_number: None,
        next_page: None,
        total_pages: None
    }
}

pub fn success(value: Value) -> Envelope {
    Envelope {
        status: EnvelopeStatus::OK,
        data: Some(value),
        error: None,
        page_number: None,
        next_page: None,
        total_pages: None
    }
}
