use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use actix_web::middleware::Next;

use crate::utils::{auth::validate_jwt, api_response::ApiResponse};

/// Middleware that checks for a valid JWT token in the Authorization header.
/// If the token is valid, it attaches the claims to the request.
pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Get the Authorization header
    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(h) => h.to_str().unwrap_or("").trim(),
        None => {
            return Err(Error::from(ApiResponse::new(401, "Unauthorized".to_string())))
        },
    };

    // Ensure the Authorization header has the "Bearer " prefix
    if !auth_header.starts_with("Bearer ") {
        return Err(Error::from(ApiResponse::new(401, "Invalid token format".to_string())));
    }

    // Extract the token by removing the "Bearer " prefix
    let token = auth_header.trim_start_matches("Bearer ").trim();

    // Validate the JWT token and extract claims
    match validate_jwt(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims); // Attach claims to the request
        }
        Err(err_msg) => {
            return Err(Error::from(ApiResponse::new(401, err_msg.to_string())));
        }
    }

    // Proceed to the next middleware or handler
    next.call(req).await.map_err(|err| {
        // Handle errors in the next middleware/handler
        Error::from(ApiResponse::new(500, err.to_string()))
    })
}
