use {
    actix_web::web,
    crate::api::v1::user::user_routes::init_user_routes,
    crate::api::v1::car::car_routes::init_car_routes,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        web::scope("/v1/api")
            .service(init_car_routes())
            .service(init_user_routes())
    );
}
