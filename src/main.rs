mod controllers;
mod database;
mod models;
mod pre_import;
mod routes;
mod middleware;
use crate::pre_import::*;

/*use axum::{http::{HeaderValue, Method}, Router};
use database::init_pool;
use dotenvy::dotenv;
use routes::user_routes::user_routes;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;*/

use middleware::cors::cors_layer;
#[allow(unused_imports)]
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    if let Some(db_name) = db_url.rsplit('/').next() {
        print!("Connecté à la Base : {} \n", db_name);
    } else {
        print!("Connection Échouée à la Base de Données");
    }
    let pool = init_pool(&db_url).await?;

    /*let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);*/

    let app = Router::new()
        .nest("/api", user_routes(pool.clone()))
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http());

    let port = env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("localhost:{}", port);
    println!("✅ Serveur API démarré sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
