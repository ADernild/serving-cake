use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;

// Structure for the response
#[derive(Serialize)]
struct CakeResponse {
    message: String,
    slices_left: i32,
}

// Structure to hold the state
struct AppState {
    cake_slices: Mutex<i32>,
}

// Handler for the POST request
async fn get_cake(data: web::Data<AppState>) -> impl Responder {
    let mut slices = data.cake_slices.lock().unwrap();

    let response = if *slices > 0 {
        // Decrease the number of slices
        *slices -= 1;
        CakeResponse {
            message: "Here you are, enjoy this slice of cake!".to_string(),
            slices_left: *slices,
        }
    } else {
        CakeResponse {
            message: "Unfortunately there is no more cake, have a hug instead 🤗".to_string(),
            slices_left: 0,
        }
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the app state with 10 slices of cake
    let app_state = web::Data::new(AppState {
        cake_slices: Mutex::new(42),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/cake", web::post().to(get_cake))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
