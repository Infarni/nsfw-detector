use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use thiserror::Error;

use crate::dto::ErrorDto;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("error read file")]
    Read(#[from] actix_multipart::MultipartError),
    
    #[error("not photo")]
    Image(#[from] image::ImageError),

    #[error("model error")]
    Model,
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::Model => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorDto { message: self.to_string() })
    }
}
