use actix_web::{web, HttpResponse, Result};

use super::{
    service,
    models::CarDTO,

};
use uuid::Uuid;

pub async fn find_all_cars(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_single_car(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match service::find_by_id(id.into_inner(), &pool) {
        Ok(person) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, person))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn insert(
    new_car: web::Json<CarDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match service::create(new_car.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn update(
    id: web::Path<i32>,
    updated_car: web::Json<CarDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match service::update(id.into_inner(), updated_car.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

pub async fn delete(id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
