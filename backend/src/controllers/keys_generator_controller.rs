use actix_web::{HttpResponse, Responder};
use zkp_core::proof_system;
use crate::utils::utils::{serialize_proving_key, serialize_verifying_key};
use crate::models::response::Keys;

/// Generate keys using the age verification circuit setup
pub async fn generate_age_keys() -> impl Responder {
    let (pk, vk) = match proof_system::setup_age_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Age circuit setup error: {:?}", e))
        },
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Proving key serialization error: {:?}", e))
        },
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Verifying key serialization error: {:?}", e))
        },
    };

    let keys = Keys {
        proving_key: proving_key_str,
        verifying_key: verifying_key_str,
    };

    HttpResponse::Ok().json(keys)
}

/// Generate keys using the citizenship verification circuit setup.
pub async fn generate_citizenship_keys() -> impl Responder {
    let (pk, vk) = match proof_system::setup_citizenship_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Citizenship circuit setup error: {:?}", e))
        },
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Proving key serialization error: {:?}", e))
        },
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Verifying key serialization error: {:?}", e))
        },
    };

    let keys = Keys {
        proving_key: proving_key_str,
        verifying_key: verifying_key_str,
    };

    HttpResponse::Ok().json(keys)
}

/// Generate keys using the college credential verification circuit setup.
pub async fn generate_college_credential_keys() -> impl Responder {
    let (pk, vk) = match proof_system::setup_credential_verification_circuit() {
        Ok(keys) => keys,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("College credential circuit setup error: {:?}", e))
        },
    };

    let proving_key_str = match serialize_proving_key(&pk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Proving key serialization error: {:?}", e))
        },
    };

    let verifying_key_str = match serialize_verifying_key(&vk) {
        Ok(s) => s,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Verifying key serialization error: {:?}", e))
        },
    };

    let keys = Keys {
        proving_key: proving_key_str,
        verifying_key: verifying_key_str,
    };

    HttpResponse::Ok().json(keys)
}
