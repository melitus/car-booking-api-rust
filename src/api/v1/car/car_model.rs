use serde::{ Deserialize, Serialize};
use diesel::prelude::*;

$[derive(Queryable, Serialize, Deserialize)]
pub struct Car {
    pub id: Option<i32>,
    pub name: String,
    pub price: String,
    pub user_id: String,
}

// Post request body for new car
#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "car"]
pub struct NewCarDAO {
    pub name: String,
    pub price: String,
    pub user_id: String,
}

impl Car {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Person>> {
        car.order(id.asc()).load::<Person>(conn)
    }

    pub fn find_by_id(i: i32, conn: &Connection) -> QueryResult<Person> {
        car.find(i).get_result::<Person>(conn)
    }

    pub fn insert(new_car: NewCarDAO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(car)
            .values(&new_car)
            .execute(conn)
    }

    pub fn update(i: i32, updated_car: NewCarDAO, conn: &Connection) -> QueryResult<usize> {
        diesel::update(car.find(i))
            .set(&updated_car)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(car.find(i)).execute(conn)
    }
}