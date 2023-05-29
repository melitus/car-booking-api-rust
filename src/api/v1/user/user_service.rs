use actix_web::{http::StatusCode, web};
use super::user_model::*;
use uuid::Uuid;
use crate::{
    exceptions::error::ServiceError as ApiErrorResponse,
};
use crate::database::db::PostgresPool;

pub fn find_all_users (pool: &web::Data<PostgresPool>) -> Result<Vec<User>, ApiErrorResponse>{
    match User::find_all_user(&mut pool.get().unwrap()) {
        Ok(users) => Ok(users),
        Err(_) => Err(
            ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
             "Users not found".to_string(),
        )),
    }
}

pub fn find_by_id(id: Uuid, pool: &web::Data<PostgresPool>) -> Result<User, ApiErrorResponse> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("User with id {} not found", id),
        )),
    }
}

pub fn create_new_user(new_user: UserDTO, pool: &web::Data<PostgresPool>) -> Result<(), ApiErrorResponse> {
    match User::create_user(new_user, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to create a new user".to_string(),
        )),
    }
}

pub fn update(
    id: Uuid,
    updated_user: UserDTO,
    pool: &web::Data<PostgresPool>,
) -> Result<(), ApiErrorResponse> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match User::update(id, updated_user, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "User could not be updated".to_string(),
            )),
        },
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("User with id {} not found", id),
        )),
    }
}

pub fn delete(user_id: Uuid, pool: &web::Data<PostgresPool>) -> Result<(), ApiErrorResponse> {
    match User::find_by_id(user_id, &mut pool.get().unwrap()) {
        Ok(_) => match User::delete(user_id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "User could not be deleted".to_string(),
            )),
        },
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("User with id {} not found", user_id),
        )),
    }
}
