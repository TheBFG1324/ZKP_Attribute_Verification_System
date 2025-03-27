use actix_web::{web, HttpResponse, Responder};
use ark_bn254::Fr;
use zkp_core::proof_system;
use crate::utils::utils::{deserialize_proof, deserialize_verifying_key};
use crate::models::proof_verification::{
    AgeProofVerify,
    CitizenshipProofVerify,
    CollegeCredentialProofVerify,
};
use crate::models::response::ProofStatus;

// Verifies an age verification proof using the provided verifying key.
pub async fn verify_age_proof(req: web::Json<AgeProofVerify>) -> impl Responder {
    let vk = match deserialize_verifying_key(&req.verifying_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key deserialization error: {:?}", e)),
    };

    // Deserialize the proof from the Base64 string
    let proof = match deserialize_proof(&req.proof) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::BadRequest().body(format!("Proof deserialization error: {:?}", e)),
    };

    // Verify the proof with the provided minimum age as the public input
    let verified = match proof_system::verify_age(&vk, &proof, req.min_age) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Age proof verification error: {:?}", e)),
    };

    let response = ProofStatus { proof_status: verified };
    HttpResponse::Ok().json(response)
}

// Verifies a citizenship verification proof using the provided verifying key.
pub async fn verify_citizenship_proof(req: web::Json<CitizenshipProofVerify>) -> impl Responder {
    let vk = match deserialize_verifying_key(&req.verifying_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key deserialization error: {:?}", e)),
    };

    // Convert the public input (merkle_root) into a field element.
    let merkle_root = Fr::from(req.merkle_root);

    // Deserialize the proof.
    let proof = match deserialize_proof(&req.proof) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::BadRequest().body(format!("Proof deserialization error: {:?}", e)),
    };

    // Verify the citizenship proof using the merkle_root as public input.
    let verified = match proof_system::verify_citizenship(&vk, &proof, merkle_root) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Citizenship proof verification error: {:?}", e)),
    };

    let response = ProofStatus { proof_status: verified };
    HttpResponse::Ok().json(response)
}

/// Verifies a college credential verification proof using the provided verifying key.
pub async fn verify_college_credential_proof(req: web::Json<CollegeCredentialProofVerify>) -> impl Responder {
    let vk = match deserialize_verifying_key(&req.verifying_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key deserialization error: {:?}", e)),
    };

    // Convert the public input (university_public_key) to a field element
    let university_public_key = Fr::from(req.university_public_key);

    // Deserialize the proof.
    let proof = match deserialize_proof(&req.proof) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::BadRequest().body(format!("Proof deserialization error: {:?}", e)),
    };

    // Verify the college credential proof using the university public key
    let verified = match proof_system::verify_college_credential(&vk, &proof, university_public_key) {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().body(format!("College credential proof verification error: {:?}", e)),
    };

    let response = ProofStatus { proof_status: verified };
    HttpResponse::Ok().json(response)
}
