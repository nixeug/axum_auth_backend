use axum::{
    Router,
    routing::{get, post},
};

use sqlx::PgPool;
use crate::controllers::list_controller::list_users;
use crate::controllers::{add_controller::add_user, get_controller::get_user};
pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(list_users))     
        .route("/users", post(add_user))  
        .route("/users/:id", get(get_user))    
        .with_state(pool)
}
