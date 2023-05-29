use actix_web::{http::StatusCode, web};
use super::car_model::*;
use uuid::Uuid;
use crate::{
    exceptions::error::ServiceError as ApiErrorResponse,
};
use crate::database::db::PostgresPool;

pub fn find_all_cars (pool: &web::Data<PostgresPool>) -> Result<Vec<Car>, ApiErrorResponse>{
    match Car::find_all_car(&mut pool.get().unwrap()) {
        Ok(cars) => Ok(cars),
        Err(_) => Err(
            ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
             "Cars not found".to_string(),
        )),
    }
}

pub fn find_by_id(id: Uuid, pool: &web::Data<PostgresPool>) -> Result<Car, ApiErrorResponse> {
    match Car::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(car) => Ok(car),
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", id),
        )),
    }
}

pub fn insert(new_car: CarDTO, pool: &web::Data<PostgresPool>) -> Result<(), ApiErrorResponse> {
    match Car::create_car(new_car, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Unable to create a new car".to_string(),
        )),
    }
}

pub fn update(
    id: Uuid,
    updated_car: CarDTO,
    pool: &web::Data<PostgresPool>,
) -> Result<(), ApiErrorResponse> {
    match Car::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Car::update(id, updated_car, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Car could not be updated".to_string(),
            )),
        },
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", id),
        )),
    }
}

pub fn delete(car_id: Uuid, pool: &web::Data<PostgresPool>) -> Result<(), ApiErrorResponse> {
    match Car::find_by_id(car_id, &mut pool.get().unwrap()) {
        Ok(_) => match Car::delete(car_id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Car could not be deleted".to_string(),
            )),
        },
        Err(_) => Err(ApiErrorResponse::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", car_id),
        )),
    }
}
