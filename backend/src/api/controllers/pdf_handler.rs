use log::info;
use std::{collections::HashSet, f64::consts::E};

use base64::engine::general_purpose;
use image::ImageOutputFormat;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use actix_web::{get, put, delete, web::{ Data, self }, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use mime::{ Mime, APPLICATION_PDF };
use uuid::Uuid;
use pdfium_render::prelude::*;
use base64::Engine as _;
use sqlx::{self, QueryBuilder, Postgres};
use chrono::prelude::*;

use crate::{AppState, errors::PdfMetadataByIdError, service::{self, pdf::PdfService}};
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