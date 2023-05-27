use actix_web::{guard, web};
use super::car_controller::*;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cars")
            .guard(guard::fn_guard(|header| header.uri.query().is_none()))
            .route(web::get().to(car_controller::index))
            .route(web::post().to(car_controller::create)),
    )
    .route("/cars", web::get().to(car_controller::show_by_user))
    .route("/cars/{id}", web::get().to(car_controller::show_a_car));
}
