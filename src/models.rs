use serde::Serialize;

// Structure for the response
#[derive(Serialize)]
pub struct CakeResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surprise: Option<String>,
    pub slices_left: i32,
}
