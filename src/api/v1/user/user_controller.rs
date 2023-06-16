use super::user_model::UserLogin;

use {
    actix_web::{web, HttpResponse},
    uuid::Uuid,
    super::{user_model::UserDTO, user_service},
    crate::middlewares::state::AppState,
    crate::utils::api::ApiResponse,
};

pub async fn find_all_users(state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_cars = user_service::find_all_users(conn)?;
    Ok(HttpResponse::Ok().json(found_cars))
}

pub async fn find_single_user(car_id: web::Path<Uuid>,state: web::Data<AppState>) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let found_car = user_service::find_by_id(car_id.into_inner(),conn)?;
    Ok(HttpResponse::Ok().json(found_car))
}

pub async fn insert_new_user(
    new_car: web::Json<UserDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    format!("This car is called {}!", new_car.name);
    let conn = &mut state.get_conn()?;
    let car_created = user_service::create_new_user(new_car.0,conn)?;
    Ok(HttpResponse::Created().json(car_created))
}

pub async fn login(login_Info: web::Json<UserLogin>,state: web::Data<AppState>) -> ApiResponse {
    format!("This car is called {}!", login_Info.email);
    let conn = &mut state.get_conn()?;
    let user_created = user_service::login(login_Info.0,conn)?;
    Ok(HttpResponse::Created().json(user_created))
}

pub async fn update(
    car_id: web::Path<Uuid>,
    updated_car: web::Json<UserDTO>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let car_updated = user_service::update(car_id.into_inner(),updated_car.0, conn)?;
    Ok(HttpResponse::Ok().json(car_updated))
}

pub async fn delete(
    car_id: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let car_deleted = user_service::delete(car_id.into_inner(), conn)?;
    Ok(HttpResponse::Ok().json(car_deleted))
}

// pub async fn login(
//     user: web::Json<UserAuthRequest>,
//     database: web::Data<Database>,
// ) -> HttpResponse {
//     let user_entry = match database.get(&user.username) {
//         Some(value) => value,
//         None => return HttpResponse::NotFound().finish(),
//     };

//     if user.password != user_entry.password {
//         return HttpResponse::BadRequest().finish();
//     }

//     build_jwt_response(user_entry.username, user_entry.role)
// }

// fn build_jwt_response(username: String, role: UserRole) -> HttpResponse {
//     let token = match generate_token(username, role, 60 * 60) {
//         Ok(value) => value,
//         Err(_) => return HttpResponse::InternalServerError().finish(),
//     };
//     let token_response = json!(UserAuthResponse { token });

//     HttpResponse::Ok().json(token_response)
// }