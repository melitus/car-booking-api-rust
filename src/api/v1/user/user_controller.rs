use super::user_model::UserLogin;

use {
    actix_web::{web, HttpResponse},
    uuid::Uuid,
    super::{user_model::UserDTO, user_service},
    crate::middlewares::state::AppState,
    crate::utils::api::ApiResponse,
};

pub async fn find_all_users(state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_cars = user_service::find_all_users(conn)?;
    Ok(HttpResponse::Ok().json(found_cars))
}

pub async fn find_single_user(car_id: web::Path<Uuid>,state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_car = user_service::find_by_id(car_id.into_inner(),conn)?;
    Ok(HttpResponse::Ok().json(found_car))
}

pub async fn insert_new_user(
    new_user: web::Json<UserDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    format!("This user is called {}!", new_user.name);
    let conn = &mut state.get_conn()?;
    let car_created = user_service::create_new_user(new_user.0,conn)?;
    Ok(HttpResponse::Created().json(car_created))
}

pub async fn login(login_info: web::Json<UserLogin>,state: web::Data<AppState>) -> ApiResponse {
    println!("This user login is called {}!", login_info.email);
    let conn = &mut state.get_conn()?;
    let user_created = user_service::login(login_info.0,conn)?;
    Ok(HttpResponse::Created().json(user_created))
}

pub async fn update(
    car_id: web::Path<Uuid>,
    updated_car: web::Json<UserDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let car_updated = user_service::update(car_id.into_inner(),updated_car.0, conn)?;
    Ok(HttpResponse::Ok().json(car_updated))
}

pub async fn delete(
    car_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let car_deleted = user_service::delete(car_id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(car_deleted))
}
