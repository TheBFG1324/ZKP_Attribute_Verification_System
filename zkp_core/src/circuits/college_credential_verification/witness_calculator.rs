use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_bn254::Fr;

// This circuit is designed for college credential verification. In a real scenario, the university signs a hash (or commitment) 
// of the user's credential using its private key. The circuit uses a signature verification gadget that takes as public input the 
// university's public key (and possibly the user's public key) and as private witnesses the signed credential and signature. 
// It then verifies that the signature is valid for the given credential, proving that the credential was issued by the university without 
// revealing its sensitive details.
pub struct CollegeCredentialVerificationCircuit<F: PrimeField> {
    pub university_public_key: Option<F>,
    pub credential: Option<F>,
    pub signature: Option<F>
}

impl<F: PrimeField> ConstraintSynthesizer<F> for CollegeCredentialVerificationCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the public input: University Public Key
        let public_key_var = FpVar::<F>::new_input(cs.clone(), || {
            self.university_public_key.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate the private witnesses: User's Credential and Signature
        let credential_var = FpVar::<F>::new_witness(cs.clone(), || {
            self.credential.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let signature_var = FpVar::<F>::new_witness(cs.clone(), || {
            self.signature.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Simulated digital signature check
        let sum_var = &credential_var + &signature_var;
        sum_var.enforce_equal(&public_key_var)?;
        
        Ok(())
    }

}

pub fn calculate_college_credential_verification_witness(
    university_public_key: Option<Fr>,
    credential: Option<Fr>,
    signature: Option<Fr>
) -> CollegeCredentialVerificationCircuit<Fr> {
    CollegeCredentialVerificationCircuit {
        university_public_key,
        credential,
        signature,
    }
}