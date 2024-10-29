use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::seq::SliceRandom;
use serde::Serialize;
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
async fn get_cake(data: web::Data<AppState>) -> impl Responder {
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
