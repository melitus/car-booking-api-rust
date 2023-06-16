use crate::api::v1::user::user_controller;
use actix_web::{web, Scope};

pub fn register_user_routes() -> Scope {
    web::scope("/users")
        .service(
            web::resource("")
                .route(web::get().to(user_controller::find_all_users))
                .route(web::post().to(user_controller::insert_new_user)),
        )
        .service(
            web::resource("/login")
                .route(web::post().to(user_controller::login)),
        )
        .service(
            web::resource("/id/{user_id}")
                .route(web::get().to(user_controller::find_single_user))
                .route(web::put().to(user_controller::update))
                .route(web::delete().to(user_controller::delete)),
        )
}
