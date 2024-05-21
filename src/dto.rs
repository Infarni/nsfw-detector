use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use nsfw::model::Classification;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct DetectResponseDto {
    pub passed_validation: bool,
    pub percentage: f32,
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
        let percentage: f32 = (value[3].score + value[4].score) / 2.0 * 100.0;
        Self {
            passed_validation: percentage < 50.0,
            percentage: percentage,
        }
    }
}
