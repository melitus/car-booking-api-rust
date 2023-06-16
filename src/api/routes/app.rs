use {
    actix_web::web,
    crate::api::v1::user::user_routes::register_user_routes,
    crate::api::v1::car::car_routes::register_car_routes,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/v1/api")
            .service(register_car_routes())
            .service(register_user_routes())
    );
}
