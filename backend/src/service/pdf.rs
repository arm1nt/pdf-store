use actix_web::web;
use base64::Engine as _;
use base64::engine::general_purpose;
use log::trace;
use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::{api::dto::{paging::PagingDto, pdf::{PdfOverviewDto, PdfMetadataDto, PdfDto, PdfSearchDto, PdfUpdateDto}}, errors::PdfMetadataByIdError, util::PdfUploaded};
use crate::domain::service::pdf::PdfService;
use crate::domain::repository::pdf::PdfRepository;


#[derive(Clone)]
pub struct PdfServiceImpl {
    pub repository: Arc<dyn PdfRepository>,
}


#[async_trait]
impl PdfService for PdfServiceImpl {

    async fn upload(&self, to_upload: Vec<PdfUploaded>) -> Result<(), String> {
        trace!("service: upload()");

        for upload in to_upload {

            let res = self.repository.upload(upload.title, upload.filename, upload.author, upload.pages, upload.img).await;
            match res {
                Ok(_) => (),
                Err(err) => {
                    if err.as_database_error().is_some() {
                        let code = err.as_database_error().unwrap().code().unwrap();
                        if code != "23505" {
                            std::fs::remove_file(upload.path).unwrap()
                        }            
                    } else {
                        std::fs::remove_file(upload.path).unwrap();
                    }
                }
            }
        }

        Ok(())
    }


    async fn get_all(&self, paging: PagingDto) -> Result<PdfOverviewDto, String> {
        trace!("service: get_all()");

        let pdf_count_result = self.repository.total_number_of_pdfs().await;
        
        match pdf_count_result {
            Err(msg) => return Err(msg),
            _ => ()
        }

        let paged_pdfs_res = self.repository.get_pdfs_paged(&paging).await;

        match paged_pdfs_res {
            Err(msg) => return Err(msg),
            _ => ()
        }

        let paged_pdfs_res = paged_pdfs_res.unwrap();

        return Ok(PdfOverviewDto { pdfs_previews:paged_pdfs_res, count: pdf_count_result.unwrap() });
    }


    async fn get_pdf_metadata(&self, pdf_id: &Uuid) -> Result<PdfMetadataDto, PdfMetadataByIdError> {
        trace!("service: get_pdf_metadata");

        let pdf_metadata_res = self.repository.get_pdf_metadata(pdf_id).await;

        match pdf_metadata_res {
            Err(err) => return Err(err),
            _ => ()
        }

        let associated_tags = self.repository.get_associated_tags_of_pdf(pdf_id).await;

        match associated_tags {
            Err(err) => return Err(err),
            _ => ()
        }

        let pdf_metadata = pdf_metadata_res.unwrap();

        let metadata_dto = PdfMetadataDto {
            id: pdf_metadata.id,
            title: pdf_metadata.title,
            file_name: pdf_metadata.file_name,
            author: pdf_metadata.author,
            pages: pdf_metadata.pages,
            comments: pdf_metadata.comments,
            uploaded: pdf_metadata.time_added,
            last_accessed: pdf_metadata.last_accessed,
            picture: pdf_metadata.picture,
            tags: Some(associated_tags.unwrap())
        };

        return Ok(metadata_dto);

    }


    async fn get_by_id(&self, pdf_id: &Uuid) -> Result<PdfDto, String> {
        trace!("service: get_by_id()");

        let file_name_res = self.repository.get_by_id(pdf_id).await;

        match file_name_res {
            Err(msg) => return Err(msg),
            _ => ()
        }

        let path = format!("./upload/{}", file_name_res.as_ref().unwrap());
        let pdf_content = web::block(|| std::fs::read(path)).await.unwrap();
        let to_base64 = general_purpose::STANDARD.encode(&pdf_content.unwrap());

        return Ok(PdfDto { pdf: to_base64 });
    }


    async fn search(&self, search: &PdfSearchDto) -> Result<PdfOverviewDto, String> {
        trace!("service: search()");

        let search_res = self.repository.search(search).await;

        match search_res {
            Ok(search_res_deto) => Ok(search_res_deto),
            Err(msg) => Err(msg)
        }
    }


    async fn update(&self, update: PdfUpdateDto, pdf_id: &Uuid) -> Result<PdfMetadataDto, String> {
        trace!("service: update()");

        let update_res = self.repository.update(update, pdf_id).await;

        match update_res {
            Err(msg) => Err(msg),
            Ok(updated_pdf_dto) => Ok(updated_pdf_dto)
        }        
    }


    async fn delete(&self, pdf_id: &Uuid) -> Result<(), String> {
        trace!("service: delete()");

        let delete_from_db_res = self.repository.delete(pdf_id).await;

        let file_name;

        match delete_from_db_res {
            Err(msg) => return Err(msg),
            Ok(name) => file_name = name
        }

        let path = format!("./upload/{}", file_name);
        let _ = std::fs::remove_file(path); //Not too important whether this succeeds or not

        Ok(())
    }

    
}