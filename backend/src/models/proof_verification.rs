use serde::{Serialize, Deserialize};

// Request payload for verifying an age verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct AgeProofVerify {
    pub proof: String,
    pub min_age: u64,
}

// Request payload for verifying a citizenship verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct CitizenshipProofVerify {
    pub proof: String,
    pub merkle_root: u64,
}

// Request payload for verifying a college credential verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct CollegeCredentialProofVerify {
    pub proof: String,
    pub university_public_key: u64,
}
