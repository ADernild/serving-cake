use actix_web::{dev::ServiceRequest, web, HttpRequest, HttpResponse, Responder};
use std::sync::Mutex;
use rusqlite::Connection;
use crate::{models::CakeResponse, auth::check_auth, utils::get_random_surprise, db::take_slice, db::get_slices_left};

pub async fn get_cake(req: HttpRequest, conn: web::Data<Mutex<Connection>>, auth_token: web::Data<String>,) -> impl Responder {
    if check_auth(&ServiceRequest::from_request(req), &auth_token).is_err() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access"
        }));
    }
    let conn = conn.lock().unwrap();
    let surprise = get_random_surprise();
    let slices_left = get_slices_left(&conn).unwrap_or(0);
    let response = if slices_left > 0 {
        // Decrease the number of slices
        take_slice(&conn, "Here you are, enjoy this slice of cake 🎂!", surprise.as_deref()).unwrap();
        CakeResponse {
            message: "Here you are, enjoy this slice of cake 🎂!".to_string(),
            surprise,
            slices_left: slices_left - 1,
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
