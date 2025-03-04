use ark_ff::PrimeField;
use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use std::cmp::Ordering;

// Define the circuit for proving user_age >= min_age
pub struct AgeVerificationCircuit<F: PrimeField> {
    pub user_age: Option<F>,
    pub min_age: Option<F>,
}

// AgeVerificationCircuit struct implements the trait ConstraintSynthesizer
impl<F: PrimeField> ConstraintSynthesizer<F> for AgeVerificationCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the private witness (user_age) and public input (min_age)
        let user_age_var = FpVar::<F>::new_witness(cs.clone(), || {
            self.user_age.ok_or(SynthesisError::AssignmentMissing)
        })?; 
        let min_age_var = FpVar::<F>::new_input(cs.clone(), || {
            self.min_age.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Enforce user_age >= min_age
        user_age_var.enforce_cmp(&min_age_var, Ordering::Greater, true)?;

        Ok(())
    }
}

// Generates the AgeVerificationCircuit with needed parameters
pub fn calculate_age_verification_witness(user_age: u64, min_age: u64) -> AgeVerificationCircuit<Fr> {
    AgeVerificationCircuit {
        user_age: Some(Fr::from(user_age)),  
        min_age: Some(Fr::from(min_age)),  
    }
}
