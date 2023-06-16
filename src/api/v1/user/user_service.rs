use {
    uuid::Uuid,
    crate::exceptions::error::AppError,
    super::user_model::*,
    diesel::pg::PgConnection,
    crate::auth::claims::{ TokenDetails},

};

pub fn find_all_users (conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
    let user_list: Result<Vec<User>, AppError>  = User::find_all_user(conn);
    user_list
}

pub fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<User, AppError> {
    
    User::find_by_id( conn, id)
}

pub fn create_new_user(new_user: UserDTO, conn: &mut PgConnection) -> Result<TokenDetails, AppError> {
    
    User::signup(conn, &new_user)
}

pub fn login(login_info: UserLogin, conn: &mut PgConnection) -> Result<TokenDetails, AppError> {
    
    User::login(conn, login_info)
}

pub fn update(id: Uuid,updated_user: UserDTO,conn: &mut PgConnection) -> Result<User, AppError> {
    
     User::update(id, updated_user,conn)
}

pub fn delete(user_id: Uuid,conn: &mut PgConnection) -> Result<(), AppError> {
    let _deleted_user  = User::delete(user_id,conn);
    Ok(())
}
