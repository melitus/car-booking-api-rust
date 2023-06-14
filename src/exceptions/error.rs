use actix_web::{http::StatusCode, HttpResponse};
use diesel::r2d2::{Error as R2D2Error, PoolError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::convert::From;
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Error, Debug)]
pub enum AppError {
    // 401
    #[error("User not authorized for operation: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("User already exists")]
    AlreadyExists(JsonValue),
    #[error("One-time resource already used")]
    AlreadyUsed(JsonValue),
    #[error("Temporary authentication token expired")]
    TokenExpired(JsonValue),
    #[error("User awaiting confirmation")]
    Unconfirmed(JsonValue),
    #[error("Authentication credentials not provided or invalid")]
    Unauthenticated(JsonValue),
    #[error("Error with JWT")]
    JwtError(JsonValue),
    #[error("email or password incorrect")]
    InvalidCredentials(JsonValue),
    #[error("Configuration error on component")]
    ConfigurationError(JsonValue),
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::Forbidden(ref msg) => HttpResponse::Forbidden().json(msg),
            AppError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            AppError::AlreadyUsed(ref msg) => HttpResponse::BadRequest().json(msg),
            AppError::AlreadyExists(ref msg) => HttpResponse::BadRequest().json(msg),
            AppError::TokenExpired(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::Unconfirmed(ref msg) => HttpResponse::BadRequest().json(msg),
            AppError::Unauthenticated(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::JwtError(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::InvalidCredentials(ref msg) => HttpResponse::Unauthorized().json(msg),
            AppError::ConfigurationError(ref msg) => HttpResponse::InternalServerError().json(msg),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            AppError::AlreadyUsed(_) => StatusCode::BAD_REQUEST,
            AppError::TokenExpired(_) => StatusCode::UNAUTHORIZED,
            AppError::Unconfirmed(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
            AppError::Unauthorized(_) => StatusCode::FORBIDDEN,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            AppError::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<PoolError> for AppError {
    fn from(_err: PoolError) -> Self {
        AppError::InternalServerError
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            JwtErrorKind::InvalidToken => AppError::Unauthorized(json!({
                "error": "Token is invalid"
            })),
            JwtErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => AppError::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<R2D2Error> for AppError {
    fn from(_err: R2D2Error) -> Self {
        AppError::InternalServerError
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    AppError::UnprocessableEntity(json!({ "error": message }))
                } else {
                    AppError::InternalServerError
                }
            }
            DieselError::NotFound => {
                AppError::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => AppError::InternalServerError,
        }
    }
}

impl From<UuidError> for AppError {
    fn from(_err: UuidError) -> Self {
        AppError::NotFound(json!({"error":"Uuid is invalid."}))
    }
}
