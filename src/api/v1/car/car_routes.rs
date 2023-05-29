use actix_web::{ web};
use super::car_controller::*;

pub fn init_car_route() {
        web::scope("/cars")
        .service(
            web::resource("")
                .route(web::get().to(find_all_cars))
                .route(web::post().to(insert)),
        )
        .service(
            web::resource("/id/{id}")
                .route(web::get().to(find_single_car))
                .route(web::put().to(update))
                .route(web::delete().to(delete)),
        );
}

