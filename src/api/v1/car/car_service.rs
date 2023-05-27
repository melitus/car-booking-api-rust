use actix_web::{http::StatusCode, web};
use super::car_model::*;

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