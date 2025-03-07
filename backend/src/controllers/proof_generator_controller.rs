use actix_web::{web, HttpResponse, Responder};
use ark_bn254::Fr;
use zkp_core::proof_system;
use crate::utils::utils::{serialize_proof, serialize_proving_key, serialize_verifying_key};

use crate::models::response::GeneratedProof;
use crate::models::proof_generation::{
    AgeProofGenerationRequest,
    CitizenshipProofGenerationRequest,
    CollegeCredentialProofGenerationRequest,
};

/// Generates the ZK-SNARK age threshold proof and returns a `GeneratedProof` in JSON
pub async fn generate_age_proof(req: web::Json<AgeProofGenerationRequest>) -> impl Responder {
    // Setup circuit for age verification
    let (pk, vk) = match proof_system::setup_age_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Age circuit setup error: {:?}", e)),
    };

    // Generate the proof
    let proof = match proof_system::prove_age(&pk, req.user_age, req.min_age) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Age proof generation error: {:?}", e)),
    };

    // Serialize the proof and keys into Base64 strings
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key serialization error: {:?}", e)),
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        proving_key: Some(proving_key_str),
        verifying_key: Some(verifying_key_str),
    };

    HttpResponse::Ok().json(response)
}

/// Generates the ZK-SNARK citizenship status proof and returns a `GeneratedProof` in JSON
pub async fn generate_citizenship_proof(req: web::Json<CitizenshipProofGenerationRequest>) -> impl Responder {
    // Setup circuit for citizenship verification.
    let (pk, vk) = match proof_system::setup_citizenship_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Citizenship circuit setup error: {:?}", e)),
    };

    // Convert inputs to field elements.
    let merkle_root = Fr::from(req.merkle_root);
    let path = Fr::from(req.path);
    let leaf = Fr::from(req.leaf);

    // Generate the proof.
    let proof = match proof_system::prove_citizenship(&pk, Some(merkle_root), Some(path), Some(leaf)) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Citizenship proof generation error: {:?}", e)),
    };

    // Serialize the proof and keys.
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key serialization error: {:?}", e)),
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        proving_key: Some(proving_key_str),
        verifying_key: Some(verifying_key_str),
    };

    HttpResponse::Ok().json(response)
}

/// Generates the ZK-SNARK college credential status proof and returns a `GeneratedProof` in JSON.
pub async fn generate_college_credential_proof(req: web::Json<CollegeCredentialProofGenerationRequest>) -> impl Responder {
    // Setup circuit for college credential verification.
    let (pk, vk) = match proof_system::setup_credential_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => return HttpResponse::InternalServerError().body(format!("College credential circuit setup error: {:?}", e)),
    };

    // Convert inputs to field elements.
    let university_public_key = Fr::from(req.university_public_key);
    let credential = Fr::from(req.credential);
    let signature = Fr::from(req.signature);

    // Generate the proof.
    let proof = match proof_system::prove_college_credential(&pk, Some(university_public_key), Some(credential), Some(signature)) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("College credential proof generation error: {:?}", e)),
    };

    // Serialize the proof and keys.
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key serialization error: {:?}", e)),
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Verifying key serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        proving_key: Some(proving_key_str),
        verifying_key: Some(verifying_key_str),
    };

    HttpResponse::Ok().json(response)
}
