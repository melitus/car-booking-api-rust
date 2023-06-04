use crate::exceptions::error::AppError;
use actix_web::HttpResponse;

pub type ApiResponse = Result<HttpResponse, AppError>;
