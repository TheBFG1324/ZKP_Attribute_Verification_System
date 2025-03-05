use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey, ProvingKey, prepare_verifying_key};
use ark_relations::r1cs::SynthesisError;
use rand::thread_rng;

use crate::circuits::age_verification::witness_calculator::{
    AgeVerificationCircuit, calculate_age_verification_witness,
};
use crate::circuits::citizenship_verification::witness_calculator::{
    CitizenshipVerificationCircuit, calculate_citizenship_verification_witness,
};

/// Performs a one-time trusted setup for the age verification circuit
pub fn setup_age_verification_circuit() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), SynthesisError> {
    // Create a dummy circuit instance with arbitrary valid values
    let dummy_circuit = AgeVerificationCircuit {
        user_age: Some(Fr::from(20u64)),
        min_age: Some(Fr::from(18u64)),
    };

    let mut rng = thread_rng();
    
    // Generate parameters once to get the proving and verifying keys
    let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(dummy_circuit, &mut rng)?;
    Ok((params.clone(), params.vk))
}

/// Generates a proof for the circuit given a user's age and the minimum age
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

/// Verifies a given proof using the verifying key and the public input (min_age)
pub fn verify_age(
    vk: &VerifyingKey<Bn254>,
    proof: &Proof<Bn254>,
    min_age: u64,
) -> Result<bool, SynthesisError> {
    let pvk = prepare_verifying_key(vk);
    let public_input = Fr::from(min_age);
    Ok(Groth16::<Bn254>::verify_proof(&pvk, proof, &[public_input]))?
}

// Does a one time setup to generate proving and verifying keys for CitizenshipVerifictionCircuit
pub fn setup_citizenship_verification_circuit() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), SynthesisError> {
    // Generate a sample Merkle root (public input)
    let merkle_root = Some(Fr::from(123456u64));

    // Generate a dummy leaf node (hashed user ID)
    let leaf = Some(Fr::from(98765u64));

    // Generate a dummy Merkle proof (Merkle path)
    let path = Some(Fr::from(98765u64));
    
    // Sets up a dummy circuit
    let dummy_circuit = CitizenshipVerificationCircuit{
        merkle_root,
        path,
        leaf,
    };

    // Get random variable
    let mut rng = thread_rng();

    // Generate proving and verifying keys
    let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(dummy_circuit, &mut rng)?;
    Ok((params.clone(), params.vk))
}

// ZKP proof generator for citizenship status
pub fn prove_citizenship(
    proving_key: &ProvingKey<Bn254>,
    merkle_root: Option<Fr>,
    path: Option<Fr>,
    leaf: Option<Fr>,
) -> Result<Proof<Bn254>, SynthesisError> {
    // Create the circuit with public and private inputs
    let circuit = calculate_citizenship_verification_witness(merkle_root, path, leaf);
    
    // Get random variable
    let mut rng = thread_rng();
    
    // Generate the proof
    let proof = Groth16::<Bn254>::create_random_proof_with_reduction(circuit, proving_key, &mut rng)?;
    Ok(proof)
}

// Verify a given proof of a user's citizenship status
pub fn verify_citizenship(
    vk: &VerifyingKey<Bn254>,
    proof: &Proof<Bn254>,
    merkle_root: Fr, 
) -> Result<bool, SynthesisError> {
    // Prepare the verifying key
    let pvk = prepare_verifying_key(vk);

    // Check the given proof with public parameters
    Ok(Groth16::<Bn254>::verify_proof(&pvk, proof, &[merkle_root]))?
}

