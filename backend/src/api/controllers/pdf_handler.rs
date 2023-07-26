use log::info;
use actix_web::{web::{ Data, self }, HttpResponse, Responder};
use actix_multipart::form::MultipartForm;
use uuid::Uuid;

use crate::{AppState, errors::PdfMetadataByIdError, service::pdf::PdfService, api::dto::pdf::{PdfSearchDto, PdfUpdateDto}, util::{UploadForm, map_pdfs}};
use crate::api::dto::paging::PagingDto;
use crate::api::dto::error::ErrorDto;


pub async fn get_all(state: Data<AppState>, paging: web::Query<PagingDto>) -> impl Responder {
    info!("get_all()");

    if paging.page.is_none() || paging.size.is_none() {
        let error_response = ErrorDto {
            message: "Page number and page size must be provided".to_string()
        };
        return HttpResponse::BadRequest().json(error_response);
    }

    let paged_pds_res = state.service.get_all(PagingDto { size: paging.size, page: paging.page }).await;

    match paged_pds_res {
        Ok(paged_pdfs) => HttpResponse::Ok().json(paged_pdfs),
        Err(msg) => HttpResponse::InternalServerError().json(ErrorDto { message: msg })
    }
}



pub async fn get_metadata_by_id(state: Data<AppState>, id: web::Path<String>) -> impl Responder {
    info!("get_metadata_by_id()");

    let id_string = id.into_inner();
    let pdf_id = String::as_str(&id_string);
    let pdf_id = Uuid::parse_str(pdf_id);

    match pdf_id {
        Err(_) => return HttpResponse::BadRequest().json( ErrorDto { message: "Invalid pdf ID given".to_string() }),
        _ => ()
    }

    let metadata_res = state.service.get_pdf_metadata(&pdf_id.unwrap()).await;

    if metadata_res.is_ok() {
        return HttpResponse::Ok().json(metadata_res.unwrap())
    } else {
        let err = metadata_res.err().unwrap();

        match err {
            PdfMetadataByIdError::NotFound(msg) => return HttpResponse::NotFound().json(ErrorDto { message: msg }),
            PdfMetadataByIdError::DatabaseError(msg) => return HttpResponse::InternalServerError().json(ErrorDto { message: msg })
        }
    }
}



pub async fn get_by_id(state: Data<AppState>, id: web::Path<String>) -> impl Responder {
    info!("get_by_id()");

    let id_string = id.into_inner();
    let pdf_id = String::as_str(&id_string);
    let pdf_id = Uuid::parse_str(pdf_id);

    match pdf_id {
        Err(_) => return HttpResponse::BadRequest().json(ErrorDto { message: "Invalid pdf ID given".to_string() }),
        _ => ()
    }

    let pdf_res = state.service.get_by_id(&pdf_id.unwrap()).await;

    match pdf_res {
        Ok(pdf_dto) => return HttpResponse::Ok().json(pdf_dto),
        Err(msg) => return HttpResponse::InternalServerError().json(ErrorDto { message: msg })
    }
}



pub async fn search(state: Data<AppState>, search: web::Query<PdfSearchDto>) -> impl Responder {
    info!("search()");

    if search.page.is_none() || search.size.is_none() {
        return HttpResponse::BadRequest().json(ErrorDto { message: "Paging information is required".to_string() });
    }

    if search.title.is_none() && search.author.is_none() && search.tag.is_none() {
        return HttpResponse::BadRequest().json(ErrorDto { message: "Search parameters are required".to_string() });
    }

    let search_dto = search.into_inner();

    let search_res = state.service.search(&search_dto).await;

    match search_res {
        Ok(search_res_dto) => return HttpResponse::Ok().json(search_res_dto),
        Err(msg) => return HttpResponse::InternalServerError().json(ErrorDto { message: msg })
        
    }
}



pub async fn update(state: Data<AppState>, update: web::Json<PdfUpdateDto>, id: web::Path<String>) -> impl Responder {
    info!("update()");

    let id_string = id.into_inner();
    let pdf_id = String::as_str(&id_string);
    let pdf_id = Uuid::parse_str(pdf_id);

    match pdf_id {
        Err(_) => return HttpResponse::BadRequest().json(ErrorDto { message: "Invalid pdf ID given".to_string() }),
        _ => ()
    }

    let update = update.into_inner();

    let update_res = state.service.update(update, &pdf_id.unwrap()).await;

    match update_res {
        Ok(updated_pdf_dto) => HttpResponse::Ok().json(updated_pdf_dto),
        Err(msg) => HttpResponse::InternalServerError().json(ErrorDto { message: msg })
    }

}



pub async fn delete(state: Data<AppState>, id: web::Path<String>) -> impl Responder {
    info!("delete()");

    let id_string = id.into_inner();
    let pdf_id = String::as_str(&id_string);
    let pdf_id = Uuid::parse_str(pdf_id);

    match pdf_id {
        Err(_) => return HttpResponse::BadRequest().json(ErrorDto { message: "Invalid pdf ID given".to_string() }),
        _ => ()
    }

    let delete_res = state.service.delete(&pdf_id.unwrap()).await;

    match delete_res {
        Ok(_) => HttpResponse::Ok().json(()),
        Err(msg) => HttpResponse::InternalServerError().json(ErrorDto { message: msg })
    }
}



pub async fn upload(state: Data<AppState>, MultipartForm(form): MultipartForm<UploadForm>,) -> impl Responder {
    info!("upload()");

    let mapped_pdfs = map_pdfs(MultipartForm(form));

    if mapped_pdfs.is_err() {
        return HttpResponse::InternalServerError().json(ErrorDto { message: mapped_pdfs.err().unwrap() });
    }

    let res  = state.service.upload(mapped_pdfs.unwrap()).await;

    match res {
        Ok(_) => HttpResponse::Created().json(()),
        Err(msg) => HttpResponse::InternalServerError().json(ErrorDto { message: msg })   
    }
}