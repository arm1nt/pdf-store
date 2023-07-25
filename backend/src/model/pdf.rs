use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Pdf {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub file_name: String,
    pub author: Option<String>,
    pub pages: Option<i32>,
    pub comments: Option<String>,
    pub time_added: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub picture: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfContent {
    pub pdf: String
}

#[derive(Deserialize, Debug)]
pub struct PdfPaging {
    pub size: Option<i32>,
    pub page: Option<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Search {
    pub title: Option<String>,
    pub author: Option<String>,
    pub tag: Option<String>,
    pub size: Option<i32>,
    pub page: Option<i32>
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct PdfOverview {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub picture: Option<String>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TotalPageNumber {
    pub count: Option<i64> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfOverviewInfo {
    pub pdfs_previews: Vec<PdfOverview>,
    pub count: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfUpdate {
    pub id: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub comments: Option<String>,
    pub tags: Option<Vec<String>>,
    pub picture: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfDetails {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub file_name: String,
    pub author: Option<String>,
    pub pages: Option<i32>,
    pub comments: Option<String>,
    pub uploaded: Option<chrono::DateTime<chrono::Utc>>,
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
    pub picture: Option<String>,
    pub tags: Option<Vec<String>>
}