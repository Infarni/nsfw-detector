use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use nsfw_detector::{handlers, state};
use state::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::detect_photo_handler),
    components(schemas(
        nsfw_detector::dto::DetectResponseDto,
        nsfw_detector::dto::ErrorDto,
        nsfw_detector::dto::FileDto,
        nsfw_detector::dto::ClassificationDto,
        nsfw_detector::dto::ClassificationName,
    ))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .expect("PORT is not set")
        .parse::<u16>()
        .expect("PORT IS NOT VALID");
    env_logger::init();

    let app_state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .app_data(web::Data::new(app_state.clone()))
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(handlers::detect_photo_handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
