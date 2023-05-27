use actix_web::{http::StatusCode, web};
use super::car_model::*;
use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,

}

pub fn find_all_cars () -> Result<Vec<Car>, Error>{
    match Car.find_all(%pool.get().unwrap()) {
        Ok(cars) => Ok(car),
        Err(_) => Err((
            ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<Car, ServiceError> {
    match Car::find_by_id(id, &pool.get().unwrap()) {
        Ok(car) => Ok(car),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", id),
        )),
    }
}

pub fn insert(new_car: NewCarDAO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Car::insert(new_person, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(
    id: i32,
    updated_car: NewCarDAO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Car::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Car::update(id, updated_car, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", id),
        )),
    }
}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Car::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match Car::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Car with id {} not found", id),
        )),
    }
}
