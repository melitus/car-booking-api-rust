use {
    uuid::Uuid,
    crate::exceptions::error::AppError,
    super::car_model::*,
    diesel::pg::PgConnection,

};


pub fn find_all_cars (conn: &mut PgConnection) -> Result<Vec<Car>, AppError> {
    let car_list: Result<Vec<Car>, AppError>  = Car::find_all_car(conn);
    car_list
}

pub fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Car, AppError> {
    
    Car::find_by_id( conn, id)
}

pub fn create_new_car(new_car: CarDTO, conn: &mut PgConnection) -> Result<Car, AppError> {
    
     Car::create_car(conn, &new_car)
}

pub fn update(id: Uuid,updated_car: CarUpdateDTO,conn: &mut PgConnection) -> Result<Car, AppError> {
    
     Car::update(id, updated_car,conn)
}

pub fn delete(car_id: Uuid,conn: &mut PgConnection) -> Result<(), AppError> {
    Car::delete(car_id,conn);
    Ok(())
}
