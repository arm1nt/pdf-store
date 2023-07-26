use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use base64::{engine::general_purpose, Engine};
use image::ImageOutputFormat;
use pdfium_render::{prelude::{Pdfium, PdfiumError, PdfDocumentMetadataTagType}, render_config::PdfRenderConfig};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}


pub fn get_preview_of_pdf(location: &String, filename: &str) -> std::result::Result<(String, String, Option<String>, Option<i32>), PdfiumError> {

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );


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

}