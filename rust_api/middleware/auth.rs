use actix_web::{dev::ServiceRequest, Error};
use actix_web::error;
use std::env;

pub async fn api_key_middleware(
    req: ServiceRequest,
    next: actix_web::dev::ServiceResponse,
) -> Result<actix_web::dev::ServiceResponse, Error> {
    // Get expected API key from environment
    let expected_key = env::var("API_KEY")
        .unwrap_or_else(|_| "default_api_key".to_string());

    // Check if the request has the correct API key
    if req.headers().get("x-api-key")
           .map_or(false, |key| key == expected_key.as_str()) {
        // API key is valid, continue with the request
        Ok(next)
    } else {
        // API key is missing or invalid
        Err(error::ErrorUnauthorized("Invalid API key"))
    }
}