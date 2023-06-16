use crate::api::v1::car::car_controller;
use actix_web::{web, Scope};

pub fn register_car_routes() -> Scope {
    web::scope("/cars")
        .service(
            web::resource("")
                // .wrap(auth::middleware::AuthCheckService::require_auth())
                .route(web::get().to(car_controller::find_all_cars))
                .route(web::post().to(car_controller::insert_new_car)),
        )
        .service(
            web::resource("/id/{car_id}")
                .route(web::get().to(car_controller::find_single_car))
                .route(web::put().to(car_controller::update))
                .route(web::delete().to(car_controller::delete)),
        )
}
