use actix_web::{ web};
use super::car_controller::*;

pub fn init_car_route() {
        web::scope("/cars")
        .service(
            web::resource("")
                .route(web::get().to(car_controller::index))
                .route(web::post().to(car_controller::create)),
        )
        .service(
            web::resource("/id/{id}")
                .route(web::get().to(car_controller::find_by_id))
                .route(web::put().to(car_controller::update))
                .route(web::delete().to(car_controller::delete)),
        )
}

