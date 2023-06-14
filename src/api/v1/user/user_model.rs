use {
    crate::schema::users::{self, dsl::*},
    chrono::NaiveDateTime,
    diesel::pg::PgConnection,
    diesel::prelude::*,
    diesel::{Insertable, QueryDsl, Queryable, RunQueryDsl},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
    crate::exceptions::error::AppError,
    crate::auth::hash::{verify_password, hash_password},
    crate::auth::jwt::{generate_token},
    crate::auth::claims::{ TokenDetails},

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
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLoggedIn {
    pub name: String,
    pub email: String,
    pub jwt: String,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}

impl User {

    pub fn generate_token(&self) -> Result<TokenDetails, AppError> {
        let token = generate_token(&self.id, &self.email)?;
        Ok(token)
    }

    pub fn find_all_user(conn: &mut PgConnection) -> Result<Vec<Self>, AppError> {
        let list = users::table.load::<Self>(conn)?;
        Ok(list)
    }

    pub fn find_by_id(conn: &mut PgConnection, car_id: Uuid) -> Result<Self, AppError> {
        let car = users::table.find(car_id).first(conn)?;
        Ok(car)
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

    pub fn find_user_by_email(conn: &mut PgConnection, user_email: String) -> Result<Self, AppError> {
        let result = users.filter(email.eq(user_email)).first::<User>(conn)?;
        Ok(result)
    }

pub fn signup<'a>( conn: &mut PgConnection, new_user: &UserDTO) -> Result<TokenDetails, AppError> {
    use diesel::prelude::*;
    let hashed_password = hash_password(new_user.password.as_bytes());

    let record = UserDTO {
        name: new_user.name.to_string(),
        email: new_user.email.to_string(),
        password: hashed_password,
    };

    let user = diesel::insert_into(users::table)
        .values(&record)
        .get_result::<User>(conn)?;

    let token = user.generate_token()?;
    Ok(token)
}

pub fn signin(conn: &mut PgConnection,login_info:UserLogin ) -> Result<(User, TokenDetails), AppError> {
    let user = users::table
        .filter(users::email.eq(login_info.email))
        .limit(1)
        .first::<User>(conn)?;
    verify_password(&login_info.password, user.password.as_bytes());
    let token = user.generate_token()?;
    Ok((user, token))
}


}