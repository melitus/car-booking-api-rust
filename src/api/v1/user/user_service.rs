use {
    uuid::Uuid,
    crate::exceptions::error::AppError,
    super::user_model::*,
    diesel::pg::PgConnection,

};

pub fn find_all_users (conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
    let user_list: Result<Vec<User>, AppError>  = User::find_all_user(conn);
    user_list
}

pub fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<User, AppError> {
    let found_user  = User::find_by_id( conn, id);
    found_user
}

pub fn create_new_user(new_user: UserDTO, conn: &mut PgConnection) -> Result<User, AppError> {
    let created_User  = User::signup(conn, &new_user);
    created_User
}

pub fn update(id: Uuid,updated_user: UserDTO,conn: &mut PgConnection) -> Result<User, AppError> {
    let updated_user  = User::update(id, updated_user,conn);
     updated_user
}

pub fn delete(user_id: Uuid,conn: &mut PgConnection) -> Result<(), AppError> {
    let updated_user  = User::delete(user_id,conn);
    Ok(())
}
