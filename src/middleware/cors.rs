use crate::pre_import::*;
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any) // ou .allow_origin("http://localhost:5239".parse().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
}
