use crate::api::v1::car::car_controller;
use crate::api::v1::user::user_controller;

use actix_web::web;

// pub fn config_services(cfg: &mut web::ServiceConfig) {
//     log::info!("Configuring routes...");
//     cfg.service(
//         web::scope("/v1/api")
//             // .service(ping_controller::ping)
//             .service(car_routes::init_car_route)

//             // .service(init_user_route),
//     );
// }
pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/v1/api")
            .service(
                web::scope("/cars")
                    .service(
                        web::resource("")
                            .route(web::get().to(car_controller::find_all_cars))
                            .route(web::post().to(car_controller::insert_new_car)),
                    )
                    .service(
                        web::resource("/id/{car_id}")
                            .route(web::get().to(car_controller::find_single_car))
                            .route(web::put().to(car_controller::update))
                            .route(web::delete().to(car_controller::delete)),
                    ),
            )
            .service(
                web::scope("/users")
                    .service(
                        web::resource("")
                            .route(web::get().to(user_controller::find_all_users))
                            .route(web::post().to(user_controller::insert_new_user)),
                    )
                    .service(
                        web::resource("/id/{user_id}")
                            .route(web::get().to(user_controller::find_single_user))
                            .route(web::put().to(user_controller::update))
                            .route(web::delete().to(user_controller::delete)),
                    ),
            )
    );
}
