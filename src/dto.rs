use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use nsfw::model::Classification;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct DetectResponseDto {
    pub drawings: f32,
    pub hentai: f32,
    pub neutral: f32,
    pub porn: f32,
    pub sexy: f32,
}

#[derive(Debug, MultipartForm, ToSchema)]
pub struct FileDto {
    #[schema(value_type = String, format = Binary)]
    #[multipart(limit = "512 MiB")]
    pub photo: TempFile,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDto {
    pub message: String,
}

impl From<Vec<Classification>> for DetectResponseDto {
    fn from(value: Vec<Classification>) -> Self {
        Self {
            drawings: value[0].score * 100.0,
            hentai: value[1].score * 100.0,
            neutral: value[2].score * 100.0,
            porn: value[3].score * 100.0,
            sexy: value[4].score * 100.0,
        }
    }
}
