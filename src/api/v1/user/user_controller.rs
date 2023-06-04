use {
    actix_web::{web, HttpResponse},
    uuid::Uuid,
    super::{user_model::UserDTO, user_service},
    crate::middlewares::app_state::AppState,
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
    new_car: web::Json<UserDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    format!("This car is called {}!", new_car.name);
    let conn = &mut state.get_conn()?;
    let car_created = user_service::create_new_user(new_car.0,conn)?;
    Ok(HttpResponse::Created().json(car_created))
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
