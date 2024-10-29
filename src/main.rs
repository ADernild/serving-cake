use actix_web::http::header;
use actix_web::{dev::ServiceRequest, web, App, Error, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::Serialize;
use std::env;
use std::sync::Mutex;

// Structure for the response
#[derive(Serialize)]
struct CakeResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    surprise: Option<String>,
    slices_left: i32,
}

// Structure to hold the state
struct AppState {
    cake_slices: Mutex<i32>,
    auth_token: String,
}

// Function to check authorization
fn check_auth(req: &ServiceRequest, data: &web::Data<AppState>) -> Result<(), Error> {
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if auth_header.to_str().unwrap_or_default() == data.auth_token {
            return Ok(());
        }
    }
    Err(actix_web::error::ErrorUnauthorized("Invalid authorization"))
}

// Function to get random surprise message with 50% chance
fn get_random_surprise() -> Option<String> {
    if rand::random::<bool>() {
        // 50% chance of getting a surprise
        let surprises = vec![
            "Lucky you, you got a strawberry!",
            "There's a cherry on top!",
            "Extra chocolate sprinkles for you!",
            "This slice has rainbow frosting!",
            "You found a hidden candy in your slice!",
        ];
        Some(
            surprises
                .choose(&mut rand::thread_rng())
                .unwrap_or(&"Enjoy your cake!")
                .to_string(),
        )
    } else {
        None
    }
}

// Handler for the POST request
async fn get_cake(req: actix_web::HttpRequest, data: web::Data<AppState>) -> impl Responder {
    if check_auth(&ServiceRequest::from_request(req), &data).is_err() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access"
        }));
    }

    let mut slices = data.cake_slices.lock().unwrap();
    let surprise = get_random_surprise();

    let response = if *slices > 0 {
        // Decrease the number of slices
        *slices -= 1;
        CakeResponse {
            message: "Here you are, enjoy this slice of cake!".to_string(),
            surprise,
            slices_left: *slices,
        }
    } else {
        CakeResponse {
            message: "Unfortunately there is no more cake, have a hug instead 🤗".to_string(),
            surprise: None,
            slices_left: 0,
        }
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let auth_token = env::var("AUTH_TOKEN").unwrap_or_default();

    // Initialize the app state with 42 slices of cake and auth header
    let app_state = web::Data::new(AppState {
        cake_slices: Mutex::new(42),
        auth_token,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/slice", web::post().to(get_cake))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
