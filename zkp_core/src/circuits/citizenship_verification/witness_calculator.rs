use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::*;
use ark_bn254::Fr;

// A circuit simulating a “Merkle check” via a dummy Poseidon hash: hash(path, leaf) == merkle_root
pub struct CitizenshipVerificationCircuit {
    pub merkle_root: Option<Fr>,
    pub path: Option<Fr>,
    pub leaf: Option<Fr>,
}

// A *dummy* poseidon_hash_gadget
fn poseidon_hash_gadget(
    inputs: Vec<FpVar<Fr>>,
    _params_var: &FpVar<Fr>,
) -> Result<FpVar<Fr>, SynthesisError> {
    
    // For demonstration, "hash" = sum(inputs).
    let mut acc = FpVar::constant(Fr::from(0));
    for inp in inputs {
        acc += inp;
    }
    Ok(acc)
}

impl ConstraintSynthesizer<Fr> for CitizenshipVerificationCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate the root as a public input
        let root_var = FpVar::new_input(cs.clone(), || {
            self.merkle_root.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate path as a private witness
        let path_var = FpVar::new_witness(cs.clone(), || {
            self.path.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate leaf as a private witness
        let leaf_var = FpVar::new_witness(cs.clone(), || {
            self.leaf.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // A dummy "Poseidon params" variable (constant in circuit). In a real circuit, you'd store real Poseidon parameters in the struct
        let poseidon_params_var = FpVar::new_constant(cs.clone(), Fr::from(9999u64))?;

        // "Hash" the path and leaf
        let hash_var = poseidon_hash_gadget(
            vec![path_var, leaf_var],
            &poseidon_params_var,
        )?;

        // Enforce hash == root, which is our dummy merkle proof check
        hash_var.enforce_equal(&root_var)?;

        Ok(())
    }
}

/// Helper to build the circuit instance
pub fn calculate_citizenship_verification_witness(
    merkle_root: Option<Fr>,
    path: Option<Fr>,
    leaf: Option<Fr>,
) -> CitizenshipVerificationCircuit {
    CitizenshipVerificationCircuit {
        merkle_root,
        path,
        leaf,
    }
}
