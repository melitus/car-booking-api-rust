use crate::database::db::PostgresPool;
use crate::utils::response::ResponseBody;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use super::{user_model::UserDTO, user_service};

pub async fn find_all_users(pool: web::Data<PostgresPool>) -> Result<HttpResponse> {
    match user_service::find_all_users(&pool) {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", users))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_single_user(
    user_id: web::Path<Uuid>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match user_service::find_by_id(user_id.into_inner(), &pool) {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", user))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn insert_new_user(
    new_user: web::Json<UserDTO>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    format!("This car is called {}!", new_user.email);
    // print!("new car {:?}", new_car);
    match user_service::create_new_user(new_user.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn update(
    user_id: web::Path<Uuid>,
    updated_user: web::Json<UserDTO>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match user_service::update(user_id.into_inner(), updated_user.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn delete(
    user_id: web::Path<Uuid>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match user_service::delete(user_id.into_inner(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}
