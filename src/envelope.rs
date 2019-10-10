use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
pub enum EnvelopeStatus {
    OK,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct EnvelopeError {
    pub code: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub status: EnvelopeStatus,
    pub data: Option<Value>,
    pub error: Option<EnvelopeError>,
    pub page_number: Option<i32>,
    pub next_page: Option<String>,
    pub total_pages: Option<i32>,
}

pub fn error(code: i32, description: String) -> Envelope {
    let error = EnvelopeError { code, description };
    Envelope {
        status: EnvelopeStatus::Error,
        data: None,
        error: Some(error),
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
