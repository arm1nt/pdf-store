use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BadRequestError {
    pub msg: String
}