use actix_web::web;
use crate::controllers::proof_generator_controller::generate_citizenship_proof;
use crate::controllers::proof_verifier_controller::verify_citizenship_proof;

// Route for proof generation and verification for citizenship status
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/citizenship")
            .route("/generate", web::post().to(generate_citizenship_proof))
            .route("/verify", web::post().to(verify_citizenship_proof)),
    );
}
