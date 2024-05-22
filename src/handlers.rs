use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use nsfw::examine;

use crate::{dto::{DetectQueryDto, DetectResponseDto}, error::AppError, state::AppState};

#[utoipa::path(
    post,
    path = "/api/v1/detector/photo",
    tag = "Detect",
    params(("trigger" = Option<f32>, Query, description = "trigger")),
    request_body(content = FileDto, description = "File to upload", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "File checked successfully", body = DetectResponseDto),
        (status = 400, description = "Invalid input data", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    )
)]
#[post("/api/v1/detector/photo")]
pub async fn detect_photo_handler(
    data: web::Data<AppState>,
    mut payload: Multipart,
    query: web::Query<DetectQueryDto>
) -> Result<HttpResponse, AppError> {
    let mut bytes: Vec<u8> = Vec::new();
    while let Some(mut field) = payload.try_next().await? {
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            bytes.extend_from_slice(&data);
        }
    }

    let image = image::load_from_memory(&bytes)?.to_rgba8();
    let result = match examine(&data.model, &image) {
        Ok(value) => value,
        Err(_) => Err(AppError::Model)?,
    };

    Ok(HttpResponse::Ok().json(DetectResponseDto::new(result, query.trigger)))
}
