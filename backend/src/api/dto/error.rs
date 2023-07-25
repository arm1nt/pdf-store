use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorDto {
    pub message: String
}