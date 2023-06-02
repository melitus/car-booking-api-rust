use {
    actix_web::{web, HttpResponse, Result},
    uuid::Uuid,
    crate::database::db::PostgresPool,
    crate::utils::response::ResponseBody,
    super::{car_model::CarDTO, car_service}
};

pub async fn find_all_cars(pool: web::Data<PostgresPool>) -> Result<HttpResponse> {
    match car_service::find_all_cars(&pool) {
        Ok(cars) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", cars))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_single_car(
    car_id: web::Path<Uuid>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match car_service::find_by_id(car_id.into_inner(), &pool) {
        Ok(car) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", car))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn insert_new_car(
    new_car: web::Json<CarDTO>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    format!("This car is called {}!", new_car.name);
    // print!("new car {:?}", new_car);
    match car_service::create_new_car(new_car.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn update(
    car_id: web::Path<Uuid>,
    updated_car: web::Json<CarDTO>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match car_service::update(car_id.into_inner(), updated_car.0, &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn delete(
    car_id: web::Path<Uuid>,
    pool: web::Data<PostgresPool>,
) -> Result<HttpResponse> {
    match car_service::delete(car_id.into_inner(), &pool) {
        Ok(()) => Ok(HttpResponse::Ok().json(ResponseBody::new("ok", ""))),
        Err(err) => Ok(err.response()),
    }
}
