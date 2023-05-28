use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::pg::PgConnection,
    diesel::{Insertable, Queryable, RunQueryDsl},
    diesel::result::Error,
    uuid::Uuid,
    chrono::NaiveDateTime,

    crate::error::AppError,
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

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "cars"]
pub struct CreateCar {
    pub name: String,
    pub price: String,
    pub user_id: String,
}

#[derive(AsChangeset, Debug, Deserialize, Clone)]
#[table_name = "cars"]
pub struct UpdateCar {
    pub name: Option<String>,
    pub price: Option<String>,
    pub user_id: Option<String>,
}

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
   
    pub fn find_all_car(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = cars::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Self, AppError> {
        let user = cars::table.find(id).first(conn)?;
        Ok(user)
    }

    pub fn create_car(
        conn: &mut PgConnection,
        records: Vec<CreateCar>,
    ) -> Result<Vec<Self>, AppError> {
        let car = diesel::insert_into(cars::table)
            .values(records)
            .get_results::<Tag>(conn)?;
        Ok(car)
    }


    pub fn update(
        conn: &mut PgConnection,
        car_id: Uuid,
        changeset: UpdateCar,
    ) -> Result<Self, AppError> {
        let target = cars::table.filter(cars::id.eq(car_id));
        let car = diesel::update(target)
            .set(changeset)
            .get_result::<Car>(conn)?;
        Ok(car)
    }
    
    pub fn delete(conn: &mut PgConnection, params: &DeleteComment) -> Result<(), AppError> {
        diesel::delete(cars::table)
            .filter(cars::id.eq(params.car_id))
            .execute(conn)?;
        Ok(())
    }
}