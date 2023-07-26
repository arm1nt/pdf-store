use std::{sync::Arc, collections::HashSet};

use chrono::Local;
use log::trace;
use sqlx::{Pool, Postgres, QueryBuilder, PgConnection};
use uuid::Uuid;

use crate::{model::pdf::{PdfOverview, Pdf, Tag, TotalPageNumber}, api::dto::{paging::PagingDto, pdf::{PdfSearchDto, PdfOverviewDto, PdfUpdateDto, PdfMetadataDto}}};
use crate::errors::PdfMetadataByIdError;

use async_trait::async_trait;

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


    async fn get_associated_tags_of_pdf_with_connection(&self, pdf_id: &Uuid, conn: &mut PgConnection) -> Result<Vec<String>, PdfMetadataByIdError> {
        trace!("repository: get_associated_tags_of_pdf()");

        let tags_query_res = sqlx::query_as!(
            Tag,
            "SELECT name FROM tags_to_pdfs WHERE id = $1",
            pdf_id
        )
        .fetch_all(conn)
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


    async fn search(&self, search: &PdfSearchDto) -> Result<PdfOverviewDto, String> {
        trace!("repository: search()");

        let size: i64 = search.size.unwrap() as i64;
        let page: i64 = size * (search.page.unwrap() - 1) as i64; 

        let search_query = String::from(
            "
            SELECT DISTINCT pdfs.id, pdfs.title, pdfs.picture, pdfs.time_added FROM pdfs LEFT JOIN tags_to_pdfs ON pdfs.id = tags_to_pdfs.id  
            WHERE
            ($1 IS NULL OR pdfs.title ILIKE CONCAT('%', $1, '%'))
            AND ($2 IS NULL OR pdfs.author ILIKE CONCAT('%', $2, '%'))
            AND ($3 IS NULL OR tags_to_pdfs.name ILIKE CONCAT('%', $3, '%'))
            ORDER BY pdfs.time_added, pdfs.id LIMIT $4 OFFSET $5
            "
        );

        let count_query = String::from(
            "
            SELECT count(DISTINCT pdfs.id) FROM pdfs LEFT JOIN tags_to_pdfs ON pdfs.id = tags_to_pdfs.id  
            WHERE
            ($1 IS NULL OR pdfs.title ILIKE CONCAT('%', $1, '%'))
            AND ($2 IS NULL OR pdfs.author ILIKE CONCAT('%', $2, '%'))
            AND ($3 IS NULL OR tags_to_pdfs.name ILIKE CONCAT('%', $3, '%'))
            "
        );

        let search_pfd_res = sqlx::query_as::<_,PdfOverview>(&search_query)
            .bind(search.title.to_owned())
            .bind(search.author.to_owned())
            .bind(search.tag.to_owned())
            .bind(size)
            .bind(page)
            .fetch_all(self.pool.as_ref())
            .await;

        match search_pfd_res {
            Err(_) => return Err("An error occured searching the pdfs".to_string()),
            _ => ()
        }

        let search_count_res = sqlx::query_as::<_,TotalPageNumber>(&count_query)
            .bind(search.title.to_owned())
            .bind(search.author.to_owned())
            .bind(search.tag.to_owned())
            .bind(size)
            .bind(page)
            .fetch_one(self.pool.as_ref())
            .await;

        match search_count_res {
            Err(_) => return Err("An error occured searching the pdfs".to_string()),
            _ => ()   
        }


        return Ok(PdfOverviewDto { pdfs_previews: search_pfd_res.unwrap(), count: search_count_res.unwrap().count });
    }


    async fn update(&self, update: PdfUpdateDto, pdf_id: &Uuid) -> Result<PdfMetadataDto, String> {
        trace!("repository: update()");

        const BIND_LIMIT: usize = 65535;

        let request_tags_cloned = update.tags.unwrap_or_else(Vec::new).clone();
        let number_of_tags_in_request = request_tags_cloned.len();

        //Start transaction
        let mut conn = self.pool.begin().await.unwrap();

        //Get all tags that we want to add and already have persisted in the database
        let persisted_tags = sqlx::query_as!(
            Tag,
            "SELECT * FROM tags WHERE name = ANY($1)",
            &request_tags_cloned[..]
        )
        .fetch_all(&mut *conn)
        .await;

        match persisted_tags {
            Err(_) => return Err("Error updating pdf".to_string()),
            _ => ()
        }


        let tags_to_add_set: HashSet<String> = request_tags_cloned.into_iter().collect();

        let persisted_tags_iterator: Vec<String> = persisted_tags.unwrap().into_iter().map(|t| t.name.unwrap().clone()).collect();
        let persisted_tags_set: HashSet<String> = HashSet::from_iter(persisted_tags_iterator.iter().cloned());

        let mut tags_to_persist_set: HashSet<&String> = HashSet::new();
        for item in tags_to_add_set.iter() {
            if !persisted_tags_set.contains(item) {
                tags_to_persist_set.insert(item);
            }
        }

        //Delete all relations between pdf and tags as we establish new ones anyways
        let delete_existing_tag_relations = sqlx::query!(
            "DELETE FROM tags_to_pdfs WHERE id = $1",
            pdf_id
        )
        .fetch_optional(&mut *conn)
        .await;

        match delete_existing_tag_relations {
            Err(_) => return Err("Error updating pdf".to_string()),
            _ => ()
        }

        if number_of_tags_in_request > 0 {

            if tags_to_persist_set.len() > 0 {
    
                let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO tags(name) ");
    
                query_builder.push_values(tags_to_persist_set.into_iter().take(BIND_LIMIT), |mut b, tag| {
                    b.push_bind(tag);
                });
                query_builder.push(";");
          
                let insert_tags_query = query_builder.build();
    
                let insert_tags_res = insert_tags_query.execute(&mut *conn).await;
    
                match insert_tags_res { 
                    Err(_) => return Err("Error updating the pdf details".to_string()),
                    _ => ()
                }  
            }
    
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("INSERT INTO tags_to_pdfs(name, id) ");
    
            query_builder.push_values(tags_to_add_set.iter().take(BIND_LIMIT), |mut b, tag | {
                b.push_bind(tag)
                .push_bind(pdf_id);
    
            });
            query_builder.push(";");
    
            let insert_tag_pdf_relation_query = query_builder.build();
            let insert_tag_pdf_relation_result = insert_tag_pdf_relation_query.execute(&mut *conn).await;
    
            match insert_tag_pdf_relation_result {
                Err(_) => return Err("Error updating the pdf details".to_string()),
                    _ => ()
            }
        }

        let update_pdf_info_result = sqlx::query_as!(
            Pdf,
            "UPDATE pdfs SET title = $2, author = $3, comments = $4, picture = $5 WHERE id = $1 RETURNING *",
            pdf_id,
            update.title,
            update.author,
            update.comments,
            update.picture
        )
        .fetch_one(&mut *conn)
        .await;

        match update_pdf_info_result {
            Err(_) => return Err("Error updating pdf".to_string()),
            _ => ()
        }

        let associated_tags_res = self.get_associated_tags_of_pdf_with_connection(pdf_id, &mut *conn).await;

        match associated_tags_res {
            Err(_) => return Err("Error updating pdf".to_string()),
            _ => ()
        }

        let transaction_result = conn.commit().await;

        match transaction_result {
            Err(_) => return Err("Error updating pdf".to_string()),
            _ => ()
        }

        let pdf_metadata = update_pdf_info_result.unwrap();

        let return_dto = PdfMetadataDto {
            id: pdf_metadata.id,
            title: pdf_metadata.title,
            file_name: pdf_metadata.file_name,
            author: pdf_metadata.author,
            pages: pdf_metadata.pages,
            comments: pdf_metadata.comments,
            uploaded: pdf_metadata.time_added,
            last_accessed: pdf_metadata.last_accessed,
            picture: pdf_metadata.picture,
            tags: Some(associated_tags_res.unwrap())
        };


        Ok(return_dto)
    }

    async fn delete(&self, pdf_id: &Uuid) -> Result<String, String> {
        trace!("repository: delete()");

        let file_name_to_delete_res = sqlx::query!(
            "SELECT file_name FROM pdfs where id=$1",
            pdf_id
        )
        .fetch_one(self.pool.as_ref())
        .await;

        match file_name_to_delete_res {
            Err(_) => return Err("Failed to delete pdf".to_string()),
            _ => ()
        }

        let delete_result = sqlx::query!(
            "DELETE FROM pdfs WHERE id=$1",
            pdf_id
        )
        .execute(self.pool.as_ref())
        .await;

        match delete_result {
            Ok(_) => Ok(file_name_to_delete_res.unwrap().file_name),
            Err(_)=> Err("Failed to delete pdf".to_string())
        }
    }

    async fn upload(&self, title: String, filename: String, author: Option<String>, pages: Option<i32>, img: String) -> Result<Pdf, sqlx::Error> {
        trace!("repository: upload()");

        let current_time = Local::now();

        let upload_query_res = sqlx::query_as!(
            Pdf,
            "INSERT INTO pdfs (title, file_name,author, pages, time_added, picture) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
            title,
            filename,
            author,
            pages,
            current_time,
            img
        )
        .fetch_one(self.pool.as_ref())
        .await;

        match upload_query_res {
            Ok(uploaded_pdf) => Ok(uploaded_pdf),
            Err(err) => Err(err)
        }

    }
    

}