use actix_web::{dev::ServiceRequest, web, HttpRequest, HttpResponse, Responder};
use crate::{models::CakeResponse, models::AppState, auth::check_auth, utils::get_random_surprise};

pub async fn get_cake(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
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
            message: "Here you are, enjoy this slice of cake 🎂!".to_string(),
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
