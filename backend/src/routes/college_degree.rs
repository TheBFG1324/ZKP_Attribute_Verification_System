use actix_web::web;
use crate::controllers::proof_generator_controller::generate_college_credential_proof;
use crate::controllers::proof_verifier_controller::verify_college_credential_proof;

// Route for proof generation and verification for college degree status
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/college_degree")
            .route("/generate", web::post().to(generate_college_credential_proof))
            .route("/verify", web::post().to(verify_college_credential_proof)),
    );
}
