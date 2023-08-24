use async_trait::async_trait;
use uuid::Uuid;

use crate::{api::dto::{paging::PagingDto, pdf::{PdfOverviewDto, PdfMetadataDto, PdfDto, PdfSearchDto, PdfUpdateDto}}, errors::PdfMetadataByIdError, util::PdfUploaded};

#[async_trait]
pub trait PdfService: Sync + Send {
    async fn get_all(&self, paging: PagingDto) -> Result<PdfOverviewDto, String>;

    async fn get_pdf_metadata(&self, pdf_id: &Uuid) -> Result<PdfMetadataDto, PdfMetadataByIdError>;

    async fn get_by_id(&self, pdf_id: &Uuid) -> Result<PdfDto, String>;

    async fn search(&self, search: &PdfSearchDto) -> Result<PdfOverviewDto, String>;

    async fn update(&self, update: PdfUpdateDto, pdf_id: &Uuid) -> Result<PdfMetadataDto, String>;

    async fn delete(&self, pdf_id: &Uuid) -> Result<(), String>;

    async fn upload(&self, to_upload: Vec<PdfUploaded>) -> Result<(), String>;
}