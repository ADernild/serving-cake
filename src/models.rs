use serde::Serialize;
use std::sync::Mutex;

// Structure for the response
#[derive(Serialize)]
pub struct CakeResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surprise: Option<String>,
    pub slices_left: i32,
}

// Structure to hold the state
pub struct AppState {
    pub cake_slices: Mutex<i32>,
    pub auth_token: String,
}
