use {
    actix_web::{web, HttpResponse},
    uuid::Uuid,
    super::{car_model::*, car_service},
    crate::middlewares::state::AppState,
    crate::utils::api::ApiResponse,
    // crate::middlewares::auth::AuthorizationService,

};

pub async fn find_all_cars(state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_cars = car_service::find_all_cars(conn)?;
    Ok(HttpResponse::Ok().json(found_cars))
}

pub async fn find_single_car(car_id: web::Path<Uuid>,state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_car = car_service::find_by_id(car_id.into_inner(),conn)?;
    Ok(HttpResponse::Ok().json(found_car))
}

pub async fn insert_new_car(
    new_car: web::Json<CarDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    format!("This car is called {}!", new_car.name);
    let conn = &mut state.get_conn()?;
    let car_created = car_service::create_new_car(new_car.0,conn)?;
    Ok(HttpResponse::Ok().json(car_created))
}

pub async fn update(
    car_id: web::Path<Uuid>,
    updated_car: web::Json<CarUpdateDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let car_updated = car_service::update(car_id.into_inner(),updated_car.0, conn)?;
    Ok(HttpResponse::Ok().json(car_updated))
}

pub async fn delete(
    car_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    car_service::delete(car_id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(()))
}
