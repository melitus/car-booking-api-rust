use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::pg::PgConnection,
    diesel::{Insertable, Queryable, RunQueryDsl},
    uuid::Uuid,
    chrono::NaiveDateTime,
    crate::schema::cars::{self, dsl::*},
    crate::exceptions::error::AppError,

};

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

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "cars"]
pub struct CarDTO {
    pub name: String,
    pub price: String,
    pub user_id: Uuid,
}

#[derive(AsChangeset,Debug, Serialize, Deserialize)]
#[table_name = "cars"]
pub struct CarUpdateDTO {
    pub name: Option<String>,
    pub price: Option<String>,
    pub user_id: Option<Uuid>,
}

pub struct DeleteCar {
    pub car_id: Uuid,
}

impl Car {
    pub fn find_all_car(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = cars::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn find_by_id(conn: &mut PgConnection, car_id: Uuid) -> Result<Self, AppError> {
        let car = cars::table.find(car_id).first(conn)?;
        Ok(car)
    }

    pub fn create_car(conn: &mut PgConnection, record: &CarDTO) -> Result<Self, AppError> {
        let car_created = diesel::insert_into(cars::table)
            .values(record)
            .get_result::<Car>(conn)?;

        Ok(car_created)
    }

    pub fn update(car_id: Uuid, updated_car: CarUpdateDTO, conn:  &mut PgConnection) -> Result<Self, AppError>  {
        let updated_car = diesel::update(cars.find(car_id))
            .set(&updated_car)
            .get_result::<Car>(conn)?;
        Ok(updated_car)
    }

    pub fn delete(car_id: Uuid, conn:  &mut PgConnection) -> Result<(), AppError> {
        diesel::delete(cars.find(car_id)).execute(conn)?;
    
        Ok(())
    }
}