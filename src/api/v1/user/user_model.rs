use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::{Insertable, Queryable, QueryDsl, RunQueryDsl},
    uuid::Uuid,
    chrono::NaiveDateTime,
    crate::schema::users::{self, dsl::*},

};
use diesel::pg::PgConnection;

#[derive(Serialize,Queryable, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserDTO {
    pub email: String,
    pub password: String,
}


impl User {
    pub fn find_all_user(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users.order(id.asc()).load::<User>(conn)
    }

    pub fn find_by_id(user_id: Uuid, conn:  &mut PgConnection) -> QueryResult<User> {
        users.find(user_id).get_result::<User>(conn)
    }

    pub fn create_user(new_user: UserDTO, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)
    }

    pub fn update(user_id: Uuid, updated_user: UserDTO, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::update(users.find(user_id))
            .set(&updated_user)
            .execute(conn)
    }

    pub fn delete(user_id: Uuid, conn:  &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(users.find(user_id)).execute(conn)
    }
}