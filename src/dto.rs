use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use log::info;
use nsfw::model::Classification;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DetectQueryDto {
    pub trigger: Option<f32>
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetectResponseDto {
    pub passed_validation: bool,
    pub percentage: f32,
    pub classification: ClassificationDto,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClassificationDto {
    pub name: ClassificationName,
    pub trigger: f32,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum ClassificationName {
    Porn,
    Sexy,
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

impl DetectResponseDto {
    pub fn new(value: Vec<Classification>, trigger: Option<f32>) -> Self {
        info!("{:#?}", value);
        let coefficient: f32 = value[3].score.max(value[4].score);
        let classification: ClassificationDto;

        if coefficient == value[3].score {
            classification = ClassificationDto {
                name: ClassificationName::Porn,
                trigger: match trigger {
                    Some(v) => v,
                    None => 70.0
                },
            };
        } else {
            classification = ClassificationDto {
                name: ClassificationName::Sexy,
                trigger: match trigger {
                    Some(v) => v,
                    None => 50.0
                },
            };
        }

        let percentage: f32 = (coefficient * 100.0).round();

        Self {
            passed_validation: percentage < classification.trigger,
            percentage: percentage,
            classification: classification,
        }
    }
}
