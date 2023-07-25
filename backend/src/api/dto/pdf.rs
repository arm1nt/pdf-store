use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::model::pdf::PdfOverview;


#[derive(Serialize, Deserialize, Debug)]
pub struct PdfOverviewDto {
    pub pdfs_previews: Vec<PdfOverview>,
    pub count: Option<i64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfMetadataDto {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub file_name: String,
    pub author: Option<String>,
    pub pages: Option<i32>,
    pub comments: Option<String>,
    pub uploaded: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub picture: Option<String>,
    pub tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfDto {
    pub pdf: String
}

#[derive(Debug, Deserialize)]
pub struct PdfSearchDto {
    pub title: Option<String>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>
}