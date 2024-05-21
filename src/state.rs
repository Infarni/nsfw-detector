use nsfw::Model;

#[derive(Clone)]
pub struct AppState {
    pub model: Model,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            model: nsfw::create_model(
                &include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/model.onnx"))[..],
            )
            .unwrap(),
        }
    }
}
