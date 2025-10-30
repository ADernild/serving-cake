use actix_web::{dev::ServiceRequest, web, HttpRequest, HttpResponse, Responder};
use rusqlite::Connection;
use std::net::IpAddr;
use std::sync::Mutex;
use uuid::Uuid;

use crate::{
    models::CakeResponse,
    auth::check_auth,
    utils::get_random_surprise,
    db::{take_slice, get_slices_left, fetch_cake_by_uid},
    rate_limiter::RateLimiter
};

pub async fn get_cake(
    req: HttpRequest,
    conn: web::Data<Mutex<Connection>>,
    auth_token: web::Data<String>,
    rate_limiter: web::Data<RateLimiter>,
) -> impl Responder {



    let ip = req.peer_addr().map(|s| s.ip()).unwrap_or_else(|| IpAddr::from([127, 0, 0, 1]));

    if let Err(msg) = rate_limiter.check(ip) {
        return HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": msg
        }));
    }

    if check_auth(&ServiceRequest::from_request(req), &auth_token).is_err() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized access"
        }));
    }

    let conn = conn.lock().unwrap();
    let surprise = get_random_surprise();
    let slices_left = get_slices_left(&conn).unwrap_or(0);
    let uid = Uuid::new_v4().to_string();
    let response = if slices_left > 0 {
        // Decrease the number of slices
        take_slice(&conn, "Here you are, enjoy this slice of cake 🎂!", surprise.as_deref(), &uid).unwrap();
        CakeResponse {
            uid: Some(uid),
            message: "Here you are, enjoy this slice of cake 🎂!".to_string(),
            surprise,
            slices_left: slices_left - 1,
        }
    } else {
        CakeResponse {
            uid: None,
            message: "Unfortunately there is no more cake, have a hug instead 🤗".to_string(),
            surprise: None,
            slices_left: 0,
        }
    };
    HttpResponse::Ok().json(response)
}

pub async fn get_cake_by_uid(
    _req: HttpRequest,
    conn: web::Data<Mutex<Connection>>,
    uid: web::Path<String>,
) -> impl Responder {
    // Lock the database connection
    let conn = conn.lock().unwrap();

    // Fetch the cake slice by UID
    match fetch_cake_by_uid(&conn, &uid) {
        Ok((uid, message, surprise, slices_left)) => {
            let response = CakeResponse {
                uid: Some(uid),
                message,
                surprise,
                slices_left,
            };
            HttpResponse::Ok().json(response)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Cake slice not found"
            }))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch cake slice"
            }))
        }
    }
}
