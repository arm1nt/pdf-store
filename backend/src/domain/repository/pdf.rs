use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgConnection;

use crate::{domain::models::pdf::{PdfOverview, Pdf}, api::dto::{paging::PagingDto, pdf::{PdfSearchDto, PdfOverviewDto, PdfUpdateDto, PdfMetadataDto}}};
use crate::errors::PdfMetadataByIdError;

#[async_trait]
pub trait PdfRepository: Send + Sync {
    async fn total_number_of_pdfs(&self) -> Result<Option<i64>, String>;
    
    async fn get_pdfs_paged(&self, paging: &PagingDto) -> Result<Vec<PdfOverview>, String>;

    async fn get_pdf_metadata(&self, pdf_id: &Uuid) -> Result<Pdf, PdfMetadataByIdError>;

    async fn get_associated_tags_of_pdf(&self, pdf_id: &Uuid) -> Result<Vec<String>, PdfMetadataByIdError>;

    async fn get_associated_tags_of_pdf_with_connection(&self, pdf_id: &Uuid, conn: &mut PgConnection) -> Result<Vec<String>, PdfMetadataByIdError>;

    async fn get_by_id(&self, pdf_id: &Uuid) -> Result<String, String>;

    async fn search(&self, search: &PdfSearchDto) -> Result<PdfOverviewDto, String>;

    async fn update(&self, update: PdfUpdateDto, pdf_id: &Uuid) -> Result<PdfMetadataDto, String>;

    async fn delete(&self, id: &Uuid) -> Result<String, String>;

    async fn upload(&self, title: String, filename: String, author: Option<String>, pages: Option<i32>, img: String) -> Result<Pdf, sqlx::Error>;

}