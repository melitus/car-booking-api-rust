use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::{Insertable, Queryable, RunQueryDsl},
    diesel::result::Error,
    uuid::Uuid,
    chrono::NaiveDateTime,
    crate::database::DBPooledConnection,
    exceptions::error::ServiceError as ApiErrorResponse,
}

$[derive(Debug, Serialize, Deserialize)]
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
$[derive(Debug, Serialize, Deserialize)]
pub struct NewCar {
    pub name: String,
    pub price: String,
    pub user_id: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "cars"]
pub struct CarDTO {
    pub name: String,
    pub price: String,
    pub user_id: String,
}

#[derive(Deletable, Serialize, Deserialize)]
pub struct DeleteCar {
    pub id: Uuid,
}

impl Car {
    pub fn to_car_dao(&self) -> CarDAO {
        CarDAO {
            id: Uuid::parse_str(self.id.as_str().unwrap()),
            name: self.name.to_string(),
            price: self.price.to_string(),
            user_id: self.user_id.to_string(),
        }
 
    }
   
    pub fn find_all_car(conn: &DBPooledConnection) -> QueryResult<Vec<Car>> {
        let list = cars::table.load::<Self>(conn)?;
         list
    }

    pub fn find_by_id(conn:&DBPooledConnection, id: Uuid) -> QueryResult<Car> {
        let user = cars::table.find(id).first(conn)?;
        user
    }

    pub fn create_car(
        conn: &DBPooledConnection,
        new_car: Vec<CreateCar>,
    ) -> Result<Vec<Self>, ApiErrorResponse> {
        let car = diesel::insert_into(cars::table)
            .values(new_car)
            .get_results::<Car>(conn)?;
        Ok(car)
    }


    pub fn update(
        conn:&DBPooledConnection
        car_id: Uuid,
        changeset: UpdateCar,
    ) -> Result<Self, ApiErrorResponse> {
        let target = cars::table.filter(cars::id.eq(car_id));
        let car = diesel::update(target)
            .set(changeset)
            .get_result::<Car>(conn)?;
        Ok(car)
    }
    
    pub fn delete(conn: &DBPooledConnection, params: &DeleteCar) -> QueryResult<usize> {
        diesel::delete(cars::table)
            .filter(cars::id.eq(params.car_id))
            .execute(conn)?;
        Ok(())
    }
}