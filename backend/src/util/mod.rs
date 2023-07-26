use log::trace;
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use base64::{engine::general_purpose, Engine};
use image::ImageOutputFormat;
use mime::APPLICATION_PDF;
use pdfium_render::{prelude::{Pdfium, PdfiumError, PdfDocumentMetadataTagType}, render_config::PdfRenderConfig};
use std::result::Result;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

#[derive(Debug)]
pub struct PdfUploaded {
    pub title: String,
    pub filename: String,
    pub author: Option<String>,
    pub pages: Option<i32>,
    pub img: String,
    pub path: String
}


pub fn map_pdfs(MultipartForm(form): MultipartForm<UploadForm>) -> Result<Vec<PdfUploaded>, String> {
    trace!("map_pdfs()");

    let binding_res = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"));

    if binding_res.is_err() {
        return Err("Error mapping pdf".to_string());
    }

    let pdfium = &Pdfium::new(binding_res.unwrap());

    let mut pdf_to_upload: Vec<PdfUploaded> = Vec::new();
    

    for file in form.files {

        if file.content_type.is_none() {
            continue;
        }

        if !(file.content_type.unwrap() == APPLICATION_PDF) {
            continue;
        }

        let file_name = file.file_name.unwrap();
        let file_name_temp = file_name.clone();

        let path = format!("./upload/{}", file_name);
        let path_cloned = path.clone();
        let persist_file_fs = file.file.persist(path);


        if persist_file_fs.is_err() {
            continue;
        }

        let pdf_info_res = get_preview_of_pdf_pdfium(&pdfium, &path_cloned, String::as_str(&file_name_temp));

        if pdf_info_res.is_err() {
            std::fs::remove_file(path_cloned).unwrap();
            continue;
        }

        let (img, title, author, pages) = pdf_info_res.unwrap();

        pdf_to_upload.push(PdfUploaded { title: title, filename: file_name_temp, author: author, pages: pages, img: img, path: path_cloned });
    }


    Ok(pdf_to_upload)

}


fn get_preview_of_pdf_pdfium(pdfium: &Pdfium, location: &String, filename: &str) -> std::result::Result<(String, String, Option<String>, Option<i32>), PdfiumError> {
    

    let document = pdfium.load_pdf_from_file(String::as_str(location), None);

    if document.is_err() {
        return Err(document.err().unwrap());
    }
    let document = document.unwrap();

    let pages = Some(document.pages().len() as i32);
    let title = match document.metadata().get(PdfDocumentMetadataTagType::Title) {
        Some(title2) => {
            if String::from(title2.value()).trim().len() == 0 {
                String::from(filename)
            } else {
                String::from(title2.value().trim())
            }
        },
        _ => String::from(filename)
    };

    let author = match document.metadata().get(PdfDocumentMetadataTagType::Author) {
        Some(author2) => Some(String::from(author2.value())),
        _ => None
    };

    let render_config = PdfRenderConfig::new()
        .scale_page_by_factor(0.8);

    let first_page = document.pages().first();

     if first_page.is_err() {
        return Err(first_page.err().unwrap());
     }

     let first_page = first_page.unwrap();

    let rendered_image = first_page.render_with_config(&render_config).unwrap().as_image();
    let mut bytes = Vec::new();
    let _ = rendered_image.write_to(&mut std::io::Cursor::new(&mut bytes), ImageOutputFormat::Jpeg(80));
    let base64image = general_purpose::STANDARD.encode(bytes);

    Ok((base64image, title, author, pages))
    
    //Ok((String::from("no"), title, author, pages))

}