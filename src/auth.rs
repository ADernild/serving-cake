use actix_web::{dev::ServiceRequest, Error, http::header};

pub fn check_auth(req: &ServiceRequest, auth_token: &str) -> Result<(), Error> {
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if auth_header.to_str().unwrap_or_default() == auth_token {
            return Ok(());
        }
    }
    Err(actix_web::error::ErrorUnauthorized("Invalid authorization"))
}
