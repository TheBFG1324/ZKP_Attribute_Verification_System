use actix_web::web;
use crate::controllers::proof_generator_controller::generate_age_proof;
use crate::controllers::proof_verifier_controller::verify_age_proof;

// Route for proof generation and verification for age threshold
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/age_verification")
            .route("/generate", web::post().to(generate_age_proof))
            .route("/verify", web::post().to(verify_age_proof)),
    );
}
