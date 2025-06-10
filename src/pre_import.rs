pub use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json as AxumJson,
};

pub use crate::models::user::User;
pub use crate::models::newuser::NewUser;
pub use sqlx::PgPool;
pub use axum::{http::Method, Router};
pub use crate::database::init_pool;
pub use dotenvy::dotenv;
pub use crate::routes::user_routes::user_routes;
pub use std::env;
pub use tower_http::cors::{Any, CorsLayer};
pub use tower_http::trace::TraceLayer;



