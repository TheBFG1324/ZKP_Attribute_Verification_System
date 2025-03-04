use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey, ProvingKey, prepare_verifying_key};
use ark_relations::r1cs::SynthesisError;
use rand::thread_rng;
use crate::circuits::age_verification::witness_calculator::{
    AgeVerificationCircuit, calculate_age_verification_witness,
};

/// Performs a one-time trusted setup for the age verification circuit.
pub fn setup_age_verification_circuit() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), SynthesisError> {
    // Create a dummy circuit instance with arbitrary valid values.
    let dummy_circuit = AgeVerificationCircuit {
        user_age: Some(Fr::from(20u64)),
        min_age: Some(Fr::from(18u64)),
    };

    let mut rng = thread_rng();
    
    // Generate parameters once to get the proving and verifying keys
    let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(dummy_circuit, &mut rng)?;
    Ok((params.clone(), params.vk))
}

/// Generates a proof for the circuit given a user's age and the minimum age.
pub fn prove_age(
    proving_key: &ProvingKey<Bn254>,
    user_age: u64,
    min_age: u64,
) -> Result<Proof<Bn254>, SynthesisError> {
    // Returns the age verification circuit
    let circuit = calculate_age_verification_witness(user_age, min_age);
    
    let mut rng = thread_rng();

    //Generate the Zero Knowledge Proof
    let proof = Groth16::<Bn254>::create_random_proof_with_reduction(circuit, proving_key, &mut rng)?;
    Ok(proof)
}

/// Verifies a given proof using the verifying key and the public input (min_age).
pub fn verify_age(
    vk: &VerifyingKey<Bn254>,
    proof: &Proof<Bn254>,
    min_age: u64,
) -> Result<bool, SynthesisError> {
    let pvk = prepare_verifying_key(vk);
    let public_input = Fr::from(min_age);
    Ok(Groth16::<Bn254>::verify_proof(&pvk, proof, &[public_input]))?
}
