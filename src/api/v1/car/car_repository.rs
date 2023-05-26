use crate::schema::cars;
super::car_model::*;

pub struct CarRepository {
    db: PooledConnection<Connection>,
}

impl CarRepository {
    pub fn new(pool: &Pool) -> Self {
        let db = pool.get().unwrap();
        Self { db }
    }

    pub fn get_all(&self) -> Result<Vec<Car>, ServiceError> {
        Ok(cars.load(&self.db)?)
    }
    pub fn find(&self, car_id: i32) -> Result<Car, ServiceError> {
        Ok(cars.filter(cars::id.eq(car_id)).first(&self.db)?)
    }

    pub fn create(&self, new_car:NewCar) -> Result<Car, ServiceError>{
       Ok(diesel::insert_into(cars::table)
        .values(new_car)
        .get_result(&self.db)?)
    }
}