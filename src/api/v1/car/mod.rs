use actix_web::{guard, web};

pub mod car_controller;
pub mod car_model;
pub mod car_service;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cars")
            .guard(guard::fn_guard(|header| header.uri.query().is_none()))
            .route(web::get().to(car_controller::index))
            .route(web::post().to(car_controller::create)),
    )
    .route("/cars", web::get().to(car_controller::show_by_user))
    .route("/cars/{id}", web::get().to(car_controller::show_a_car));
}
