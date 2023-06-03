use {
    serde::{ Deserialize, Serialize},
    diesel::prelude::*,
    diesel::pg::PgConnection,
    diesel::{Insertable, Queryable, QueryDsl, RunQueryDsl},
    uuid::Uuid,
    chrono::NaiveDateTime,
    bcrypt::{hash, verify, DEFAULT_COST},
    crate::schema::users::{self, dsl::*},
};

#[derive(Serialize,Queryable, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)] // we're removing password from being show in the response
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

    pub fn signup(user: UserDTO, conn:  &mut PgConnection) -> Result<String, String> {
        if Self::find_user_by_email(&user.email, conn).is_err() {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UserDTO {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users).values(&user).execute(conn);
            Ok("Signup successfully".to_string())
        } else {
            Err(format!("User '{}' is already registered", &user.email))
        }
    }

    // pub fn login(login: UserDTO,  conn: &mut PgConnection) -> Option<User> {
    //     if let Ok(user_to_verify) = users
    //         .filter(email.eq(&login.email))
    //         .or_filter(email.eq(&login.email))
    //         .get_result::<User>(conn)
    //     {
    //         let verify_password = !user_to_verify.password.is_empty()
    //         && verify(&login.password, &user_to_verify.password).unwrap();
    //         if verify_password
    //         let claims = TokenClaims { id: user.id };
    //                     let token_str = claims.sign_with_key(&jwt_secret).unwrap();
    //                     HttpResponse::Ok().json(token_str)
    //                 } else {
    //                     HttpResponse::Unauthorized().json("Incorrect username or password")
    //                 }
        
    // } else {
    //     None
    // }
    

// async fn login_user_handler(
//     req: HttpRequest,
//     body: web::Json<LoginUserSchema>,
//     data: web::Data<AppState>,
// ) -> Result<impl Responder, actix_web::Error> {
//     // fetch user with provided email
//     let user = User::get_from_db_by_email(&data.db, &body.email)
//         .await
//         .http_internal_error()?
//         .ok_or("User not found")
//         .http_not_found_error("User not found")?;

//     // if provided password is not valid return a 400 Bad request response
//     if !user.password_is_correct(&body.password) {
//         return Err(ErrorUnauthorized(ErrorResponse::new(
//             "Invalid email or password",
//         )));
//     }

//     let fingerprint = extract_fingerprint_from_request(&req);

//     let mut fingerprint_option = None;

//     if let Some(fingerprint) = fingerprint {
//         let fingerprint_hash = hash_fingerprint(&fingerprint);
//         fingerprint_option = Some(fingerprint_hash);
//     }

//     // acquire mutex on access and refresh tokens encoding keys
//     let access_encoding_key = data.access_encoding_key.lock().http_internal_error()?;
//     let refresh_encoding_key = data.refresh_encoding_key.lock().http_internal_error()?;

//     // generate new access & refresh token pair
//     let access_token_details = token::generate_jwt_token(
//         user.id(),
//         data.env.access_token_max_age,
//         &access_encoding_key,
//         fingerprint_option.clone(),
//     )
//     .http_internal_error()?;

//     let refresh_token_details = token::generate_jwt_token(
//         user.id(),
//         data.env.refresh_token_max_age,
//         &refresh_encoding_key,
//         fingerprint_option.clone(),
//     )
//     .http_internal_error()?;

//     let cookies = build_auth_cookies(
//         &refresh_token_details.token,
//         "true",
//         data.env.refresh_token_max_age * 60,
//     );

//     // filter out user sensitive information
//     let user_response = filter_user_record(&user);

//     Ok(HttpResponse::Ok()
//         .cookie(cookies.refresh_token_cookie)
//         .cookie(cookies.logged_in_cookie)
//         .json(json!({"status": "success", "access_token": access_token_details.token, "user": user_response})))
// }

    pub fn find_user_by_email(un: &str, conn: &mut PgConnection) -> QueryResult<User> {
        users.filter(email.eq(un)).get_result::<User>(conn)
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
