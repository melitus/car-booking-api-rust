use {
    uuid::Uuid,
    actix_web::{web, HttpResponse, Result},
    crate::database::db::PostgresPool,
    crate::utils::response::ResponseBody,
    super::{user_model::UserDTO, user_service},
    log::{debug, error, info},

};

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
    info!("This car data: {:?}", new_user);
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
