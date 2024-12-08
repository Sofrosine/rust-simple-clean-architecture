use crate::helpers::auth::decode_basic_auth_token;
use crate::helpers::custom_error::{ErrorResponse, ResponseError};
use crate::internal::entities::auth::Claims;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage};
use jsonwebtoken::{decode, DecodingKey, Validation};

pub async fn role_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
    allowed_roles: Vec<String>, // Pass the allowed roles for this middleware
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Extract the Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_header) = auth_header.to_str() {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                let secret = std::env::var("JWT_SECRET").unwrap_or_default();

                // Decode and validate the token
                if let Ok(token_data) = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                ) {
                    let claims = token_data.claims;

                    // Check if the role is allowed
                    if allowed_roles.contains(&claims.role) {
                        // Add claims to the request for further use
                        req.extensions_mut().insert(claims);
                        return next.call(req).await; // Proceed to the next service
                    }
                }
            }
        }
    }

    // Create a structured error response using ErrorResponse
    let error_response = ErrorResponse::new(
        StatusCode::UNAUTHORIZED,
        Some("Invalid token or insufficient permissions".to_string()),
        Some("Unauthorized".to_string()),
    );

    // Return the error as an Actix error
    let response = error_response.error_response();
    Err(InternalError::from_response(
        "Unauthorized",
        response,
    )
        .into())
}


pub async fn super_admin_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_header) = auth_header.to_str() {
            if auth_header.starts_with("Basic ") {
                let token = &auth_header[6..];
                match decode_basic_auth_token(token) {
                    Ok(credentials) => {
                        // Retrieve the secret from environment
                        let expected_secret = std::env::var("BASIC_AUTH_SECRET")
                            .unwrap_or_else(|_| "".to_string());

                        if format!("{}:{}", credentials.0, credentials.1) == expected_secret {
                            // Continue with the next middleware or handler
                            return next.call(req).await;
                        } else {
                            let error_response = ErrorResponse::new(
                                StatusCode::UNAUTHORIZED,
                                Some(format!("Invalid credentials: {:?}", credentials)),
                                Some("Unauthorized".to_string()),
                            );

                            let response = error_response.error_response();
                            return Err(InternalError::from_response(
                                "Unauthorized",
                                response,
                            )
                                .into());
                        }
                    }
                    Err(e) => {
                        // Token decoding failed
                        let error_response = ErrorResponse::new(
                            StatusCode::UNAUTHORIZED,
                            Some(format!("Invalid token: {}", e)),
                            Some("Unauthorized".to_string()),
                        );

                        let response = error_response.error_response();
                        return Err(InternalError::from_response(
                            "Unauthorized",
                            response,
                        )
                            .into());
                    },
                }
            }
        }
    }
    // pre-processing
    next.call(req).await
    // post-processing
}
