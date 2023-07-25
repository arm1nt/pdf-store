use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PagingDto {
    pub size: Option<i32>,
    pub page: Option<i32>
}