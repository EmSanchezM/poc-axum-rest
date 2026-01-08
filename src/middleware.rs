use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::errors::ErrorResponse;

pub async fn handle_panic(request: Request<Body>, next: Next) -> Response {
    let response = std::panic::AssertUnwindSafe(next.run(request))
        .await;
    
    response
}

pub async fn error_handler(request: Request<Body>, next: Next) -> Response {
    let response = next.run(request).await;
    
    if response.status().is_client_error() || response.status().is_server_error() {
        let status = response.status();
        
        let headers = response.headers();
        if headers.get("content-type").is_none() {
            let error_response = match status {
                StatusCode::NOT_FOUND => ErrorResponse {
                    error: "not_found".to_string(),
                    message: "The requested resource was not found".to_string(),
                },
                StatusCode::METHOD_NOT_ALLOWED => ErrorResponse {
                    error: "method_not_allowed".to_string(),
                    message: "The HTTP method is not allowed for this resource".to_string(),
                },
                StatusCode::INTERNAL_SERVER_ERROR => ErrorResponse {
                    error: "internal_server_error".to_string(),
                    message: "An internal server error occurred".to_string(),
                },
                _ => ErrorResponse {
                    error: "error".to_string(),
                    message: format!("An error occurred: {}", status.canonical_reason().unwrap_or("Unknown error")),
                },
            };
            
            return (status, Json(error_response)).into_response();
        }
    }
    
    response
}