#[derive(Debug)]
pub enum PdfMetadataByIdError {
    NotFound(String),
    DatabaseError(String)
}