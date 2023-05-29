use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::{Insertable, Queryable, RunQueryDsl},
    uuid::Uuid,
    chrono::NaiveDateTime,
    crate::schema::cars::{self, dsl::*},

};
use diesel::pg::PgConnection;


#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Car {
    pub id: Uuid,
    pub name: String,
    pub price: String,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Post request body for new car
#[derive(Debug, Serialize, Deserialize)]
pub struct NewCar {
    pub name: String,
    pub price: String,
    pub user_id: String,
}

#[derive(Insertable, AsChangeset,Debug, Serialize, Deserialize)]
#[table_name = "cars"]
pub struct CarDTO {
    pub name: String,
    pub price: String,
    pub user_id: Uuid,
}

pub struct DeleteCar {
    pub car_id: Uuid,
}

impl Car {
    pub fn find_all_car(conn: &mut PgConnection) -> QueryResult<Vec<Car>> {
        cars.order(id.asc()).load::<Car>(conn)
    }

    pub fn find_by_id(car_id: Uuid, conn:  &mut PgConnection) -> QueryResult<Car> {
        cars.find(car_id).get_result::<Car>(conn)
    }

    pub fn create_car(new_car: CarDTO, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::insert_into(cars)
            .values(&new_car)
            .execute(conn)
    }

    pub fn update(car_id: Uuid, updated_car: CarDTO, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::update(cars.find(car_id))
            .set(&updated_car)
            .execute(conn)
    }

    pub fn delete(car_id: Uuid, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(cars.find(car_id)).execute(conn)
    }
}