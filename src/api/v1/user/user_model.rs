use {
    crate::schema::users::{self, dsl::*},
    chrono::NaiveDateTime,
    diesel::pg::PgConnection,
    diesel::prelude::*,
    diesel::{Insertable, QueryDsl, Queryable, RunQueryDsl},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    crate::exceptions::error::AppError,
};

#[derive(Serialize, Queryable, Identifiable, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip)] // we're removing password from being show in the response
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoggedIn {
    pub name: String,
    pub email: String,
    pub jwt: String,
    pub refresh_token: Option<String>,
}

impl User {

    pub fn find_all_user(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = users::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn find_by_id(conn: &mut PgConnection, car_id: Uuid) -> Result<Self, AppError> {
        let car = users::table.find(car_id).first(conn)?;
        Ok(car)
    }

    pub fn signup(conn: &mut PgConnection, record: &UserDTO) -> Result<Self, AppError> {
        let car_created = diesel::insert_into(users::table)
            .values(record)
            .get_result::<User>(conn)?;

        Ok(car_created)
    }

    pub fn update(car_id: Uuid, updated_car: UserDTO, conn:  &mut PgConnection) -> Result<Self, AppError>  {
        let updated_car = diesel::update(users.find(car_id))
            .set(&updated_car)
            .get_result::<User>(conn)?;
        Ok(updated_car)
    }

    pub fn delete(car_id: Uuid, conn:  &mut PgConnection) -> Result<(), AppError> {
        diesel::delete(users.find(car_id)).execute(conn)?;
    
        Ok(())
    }
}