use actix_web::web;
use crate::controllers::keys_generator_controller::{
    generate_age_keys,
    generate_citizenship_keys,
    generate_college_credential_keys,
};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/keys")
            .route("/age", web::get().to(generate_age_keys))
            .route("/citizenship", web::get().to(generate_citizenship_keys))
            .route("/college", web::get().to(generate_college_credential_keys))
    );
}
