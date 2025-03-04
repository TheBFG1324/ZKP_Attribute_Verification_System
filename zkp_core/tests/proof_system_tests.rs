use zkp_core::proof_system::{setup_age_verification_circuit, prove_age, verify_age};

#[test]
fn test_full_proof_verification() {
    // Generate verifying and proving keys
    let (pk, vk) = setup_age_verification_circuit().expect("Setup failed");

    // Generate the proof (user_age = 25, min_age = 18) should return valid proof
    let proof = prove_age(&pk, 25, 18).expect("Proof generation failed");
    
    // Assert that the generated zkp proof has the users age >= 18
    assert!(verify_age(&vk, &proof, 18).expect("Verification failed"));
}

#[test]
fn test_fails_when_age_is_not_greater() {
    // Generate verifying and proving keys
    let (pk, _vk) = setup_age_verification_circuit().expect("Setup failed");

    // Generates proof (should fail)
    let proof = std::panic::catch_unwind(|| prove_age(&pk, 17, 18));
    
    // Verify that the proof failed
    assert!(proof.is_err());
}
