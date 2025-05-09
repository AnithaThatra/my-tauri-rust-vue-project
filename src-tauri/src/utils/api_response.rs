use std::fmt::Display;

use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};

/// Represents an API response with a status code and body content.
#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    response_code: StatusCode,
}

impl ApiResponse {
    /// Constructs a new `ApiResponse`.
    ///
    /// # Arguments
    ///
    /// * `status_code` - The HTTP status code (e.g., 404, 500, etc.).
    /// * `body` - The response body content as a string.
    ///
    /// # Returns
    ///
    /// A new `ApiResponse` with the given status code and body.
    pub fn new(status_code: u16, body: String) -> Self {
        ApiResponse {
            status_code,
            body,
            response_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

// Implementing the `Responder` trait to return a custom HTTP response.
impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.response_code).set_body(body)
    }
}

// Implementing the `Display` trait to format the `ApiResponse` in a user-friendly way.
impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}\nStatus Code: {}", self.body, self.status_code)
    }
}

// Implementing the `ResponseError` trait to handle error responses.
impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.response_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}
