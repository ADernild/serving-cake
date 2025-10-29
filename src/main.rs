use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use std::env;

mod models;
mod utils;
mod handlers;
mod db;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = db::init_db().expect("Failed to initialize database");
    let conn = web::Data::new(Mutex::new(conn));
    let auth_token = env::var("AUTH_TOKEN").unwrap_or_default();
    let auth_token = web::Data::new(auth_token);

    HttpServer::new(move || {
        App::new()
            .app_data(conn.clone())
            .app_data(auth_token.clone())
            .route("/slice", web::post().to(handlers::get_cake))
            .route("/slice/{uid}", web::get().to(handlers::get_cake_by_uid))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
