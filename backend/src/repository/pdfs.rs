use std::sync::Arc;

use log::trace;
use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use crate::{model::pdf::{PdfPaging, PdfOverview, Pdf, Tag}, api::dto::paging::PagingDto};
use crate::errors::PdfMetadataByIdError;

use async_trait::async_trait;

#[async_trait]
pub trait PdfRepository: Send + Sync {
    async fn total_number_of_pdfs(&self) -> Result<Option<i64>, String>;
    
    async fn get_pdfs_paged(&self, paging: &PagingDto) -> Result<Vec<PdfOverview>, String>;

    async fn get_pdf_metadata(&self, pdf_id: &Uuid) -> Result<Pdf, PdfMetadataByIdError>;

    async fn get_associated_tags_of_pdf(&self, pdf_id: &Uuid) -> Result<Vec<String>, PdfMetadataByIdError>;

    async fn get_by_id(&self, pdf_id: &Uuid) -> Result<String, String>;

}

pub struct PdfRepositoryImpl {
    pub pool: Arc<Pool<Postgres>>
}

#[async_trait]
impl PdfRepository for PdfRepositoryImpl {

    async fn total_number_of_pdfs(&self) -> Result<Option<i64>, String> {
        trace!("repository: total_number_of_pdfs()");
    
        let pdf_count_result = sqlx::query!(
            "SELECT count(*) FROM pdfs"
        )
        .fetch_one(self.pool.as_ref())
        .await;

        if pdf_count_result.is_ok() {
            return Ok(pdf_count_result.unwrap().count);
        } else {
            return Err("Error getting number of pdfs".to_string());
        }
    }


    async fn get_pdfs_paged(&self, paging: &PagingDto) -> Result<Vec<PdfOverview>, String> {
        trace!("repository: get_pdfs_paged()");

        let size: i64 = paging.size.unwrap() as i64;
        let page: i64 = size * (paging.page.unwrap() - 1) as i64;

        let paged_pdfs_result = sqlx::query_as!(
            PdfOverview,
            "SELECT id, title, picture FROM pdfs ORDER BY time_added, id LIMIT $1 OFFSET $2",
            size,
            page
        )
        .fetch_all(self.pool.as_ref())
        .await;

        match paged_pdfs_result {
            Ok(pdfs) => Ok(pdfs),
            Err(_) => Err("Error retrieving paginated data".to_string())
        }
    }


    async fn get_pdf_metadata(&self, pdf_id: &Uuid) -> Result<Pdf, PdfMetadataByIdError> {
        trace!("repository: get_pdf_metadata()");

        let pdf_metadata_res = sqlx::query_as!(
            Pdf,
            "SELECT * FROM pdfs WHERE id = $1",
            pdf_id
        )
        .fetch_one(self.pool.as_ref())
        .await;

        match pdf_metadata_res {
            Ok(pdf) => return Ok(pdf),
            Err(err) => {

                if err.as_database_error().is_none() {
                    return Err(PdfMetadataByIdError::NotFound("No pdf with given ID exists".to_string()));
                } else {
                    return Err(PdfMetadataByIdError::DatabaseError("Error retrieving requested pdf".to_string()));
                }
            }
        }
    }


    async fn get_associated_tags_of_pdf(&self, pdf_id: &Uuid) -> Result<Vec<String>, PdfMetadataByIdError> {
        trace!("repository: get_associated_tags_of_pdf()");

        let tags_query_res = sqlx::query_as!(
            Tag,
            "SELECT name FROM tags_to_pdfs WHERE id = $1",
            pdf_id
        )
        .fetch_all(self.pool.as_ref())
        .await;

        let mut tags_as_vec: Vec<String> = Vec::new();

        if tags_query_res.is_ok() {

            for item in tags_query_res.unwrap().into_iter() {
                tags_as_vec.push(item.name.unwrap());
            }

            return Ok(tags_as_vec);
        }

        return Err(PdfMetadataByIdError::DatabaseError("Error retrieving the tags associated with pdf".to_string()));
    }


    async fn get_by_id(&self, pdf_id: &Uuid) -> Result<String, String> {
        trace!("repository: get_by_id()");

        let query_res = sqlx::query!(
            "SELECT * FROM pdfs WHERE id = $1",
            pdf_id
        )
        .fetch_one(self.pool.as_ref())
        .await;
        
        match query_res {
            Ok(record) => return Ok(record.file_name),
            Err(_) => return Err("Error retrieving pdf information".to_string())
        }
    }

    
}