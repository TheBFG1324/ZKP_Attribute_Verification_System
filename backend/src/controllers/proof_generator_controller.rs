use actix_web::{web, HttpResponse, Responder};
use ark_bn254::Fr;
use zkp_core::proof_system;
use crate::utils::utils::{serialize_proof, deserialize_proving_key};
use crate::models::response::GeneratedProof;
use crate::models::proof_generation::{
    AgeProofGenerationRequest,
    CitizenshipProofGenerationRequest,
    CollegeCredentialProofGenerationRequest,
};

/// Generates the ZK-SNARK age threshold proof using the provided keys.
pub async fn generate_age_proof(req: web::Json<AgeProofGenerationRequest>) -> impl Responder {
    // Deserialize the provided proving key.
    let pk = match deserialize_proving_key(&req.proving_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key deserialization error: {:?}", e)),
    };

    // Generate the proof using the provided keys.
    let proof = match proof_system::prove_age(&pk, req.user_age, req.min_age) {
        Ok(proof) => proof,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Age proof generation error: {:?}", e)),
    };

    // Serialize the proof.
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        // Echo back the provided keys.
        proving_key: Some(req.proving_key.clone()),
        verifying_key: Some(req.verifying_key.clone()),
    };

    HttpResponse::Ok().json(response)
}

/// Generates the ZK-SNARK citizenship status proof using the provided keys.
pub async fn generate_citizenship_proof(req: web::Json<CitizenshipProofGenerationRequest>) -> impl Responder {
    // Deserialize the provided proving key.
    let pk = match deserialize_proving_key(&req.proving_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key deserialization error: {:?}", e)),
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

    // Serialize the proof.
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        proving_key: Some(req.proving_key.clone()),
        verifying_key: Some(req.verifying_key.clone()),
    };

    HttpResponse::Ok().json(response)
}

/// Generates the ZK-SNARK college credential status proof using the provided keys.
pub async fn generate_college_credential_proof(req: web::Json<CollegeCredentialProofGenerationRequest>) -> impl Responder {
    // Deserialize the provided proving key.
    let pk = match deserialize_proving_key(&req.proving_key) {
        Ok(key) => key,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proving key deserialization error: {:?}", e)),
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

    // Serialize the proof.
    let proof_str = match serialize_proof(&proof) {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Proof serialization error: {:?}", e)),
    };

    let response = GeneratedProof {
        proof: proof_str,
        proving_key: Some(req.proving_key.clone()),
        verifying_key: Some(req.verifying_key.clone()),
    };

    HttpResponse::Ok().json(response)
}
