use actix_web::{web, App, HttpServer};
use std::env;

mod models;
mod auth;
mod utils;
mod handlers;

use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let auth_token = env::var("AUTH_TOKEN").unwrap_or_default();
    println!("Auth token: {}", auth_token);
    // Initialize the app state with 42 slices of cake and auth header
    let app_state = web::Data::new(AppState {
        cake_slices: std::sync::Mutex::new(42),
        auth_token,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/slice", web::post().to(handlers::get_cake))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
