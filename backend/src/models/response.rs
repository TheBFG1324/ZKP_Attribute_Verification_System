use serde::{Serialize, Deserialize};

// Represents a generated proof
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedProof {
    pub proof: String,
    pub proving_key: Option<String>,
    pub verifying_key: Option<String>,
}

// Represents the status of a proof verification
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofStatus {
    pub proof_status: bool,
}

// Represents the proving and verifying keys generated during setup
#[derive(Debug, Serialize, Deserialize)]
pub struct Keys {
    pub proving_key: String,
    pub verifying_key: String,
}
