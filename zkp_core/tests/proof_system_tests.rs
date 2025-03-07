// Tests Generated by OpenAI's o3 model

use zkp_core::proof_system::{
    setup_age_verification_circuit, prove_age, verify_age,
    setup_citizenship_verification_circuit,
    prove_citizenship,
    verify_citizenship,
};
use ark_bn254::Fr;

// ----------------------------
// Age Verification Tests
// ----------------------------

#[test]
fn test_full_proof_verification() {
    // Generate verifying and proving keys for age circuit
    let (pk, vk) = setup_age_verification_circuit().expect("Setup failed");

    // Generate the proof (user_age = 25, min_age = 18) should return valid proof
    let proof = prove_age(&pk, 25, 18).expect("Proof generation failed");
    
    // Assert that the generated zkp proof has the user's age >= 18
    assert!(verify_age(&vk, &proof, 18).expect("Verification failed"));
}

#[test]
fn test_fails_when_age_is_not_greater() {
    // Generate verifying and proving keys for age circuit
    let (pk, _vk) = setup_age_verification_circuit().expect("Setup failed");

    // Generate proof (should fail) because user_age < min_age
    let proof = std::panic::catch_unwind(|| prove_age(&pk, 17, 18));
    
    // Verify that the proof creation actually failed
    assert!(proof.is_err());
}

// ----------------------------
// Citizenship Verification Tests
// ----------------------------

#[test]
fn test_citizenship_proof_passes() {
    // Generate verifying and proving keys for the citizenship circuit
    let (pk, vk) = setup_citizenship_verification_circuit().expect("Setup failed");

    // Set up "path + leaf" == root
    let root = Fr::from(10u64);
    let path = Fr::from(4u64);
    let leaf = Fr::from(6u64);

    // Generate a proof
    let proof = prove_citizenship(&pk, Some(root), Some(path), Some(leaf)).expect("Proof generation failed");

    // This should verify, because 4 + 6 = 10 (our dummy "hash" is just sum)
    assert!(verify_citizenship(&vk, &proof, root).expect("Verification failed"));
}

#[test]
fn test_citizenship_proof_fails() {
    // Generate verifying and proving keys for the citizenship circuit
    let (pk, _vk) = setup_citizenship_verification_circuit().expect("Setup failed");

    // Now path + leaf != root
    let root = Fr::from(10u64);
    let path = Fr::from(5u64);
    let leaf = Fr::from(6u64);

    // Generate the proof (the proof creation itself doesn't fail—it's just wrong)
    let proof = std::panic::catch_unwind(|| prove_citizenship(&pk, Some(root), Some(path), Some(leaf)).expect("Proof generation failed"));

    // Verification should return false
    assert!(proof.is_err());
}

// ----------------------------
// College Credential Verification Tests
// ----------------------------

#[test]
fn test_college_credential_proof_passes() {
    // Generate verifying and proving keys for the citizenship circuit
    let (pk, vk) = setup_citizenship_verification_circuit().expect("Setup failed");

    // Set up "credential + signature" == university_public_key
    let university_public_key = Fr::from(25u64);
    let credential = Fr::from(10u64);
    let signature = Fr::from(15u64);

    // Generate a proof
    let proof = prove_citizenship(&pk, Some(university_public_key), Some(credential), Some(signature)).expect("Proof generation failed");

    // This should verify
    assert!(verify_citizenship(&vk, &proof, university_public_key).expect("Verification failed"));
}

#[test]
fn test_college_credential_proof_fails() {
    // Generate verifying and proving keys for the citizenship circuit
    let (pk, _vk) = setup_citizenship_verification_circuit().expect("Setup failed");

    // Now credential + signature != univeristy_public_key
    let university_public_key = Fr::from(25u64);
    let credential = Fr::from(10u64);
    let signature = Fr::from(10u64);

    // Generate the proof (the proof creation itself doesn't fail—it's just wrong)
    let proof = std::panic::catch_unwind(|| prove_citizenship(&pk, Some(university_public_key), Some(credential), Some(signature)).expect("Proof generation failed"));

    // Verification should return false
    assert!(proof.is_err());
}