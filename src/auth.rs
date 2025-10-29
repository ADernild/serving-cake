use actix_web::{dev::ServiceRequest, web, Error, http::header};

use crate::models::AppState;

pub fn check_auth(req: &ServiceRequest, data: &web::Data<AppState>) -> Result<(), Error> {
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if auth_header.to_str().unwrap_or_default() == data.auth_token {
            return Ok(());
        }
    }
    Err(actix_web::error::ErrorUnauthorized("Invalid authorization"))
}
