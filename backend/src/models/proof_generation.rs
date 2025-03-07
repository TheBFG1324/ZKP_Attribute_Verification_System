use serde::{Serialize, Deserialize};

// Request payload for generating an age verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct AgeProofGenerationRequest {
    pub user_age: u64,
    pub min_age: u64,
}

// Request payload for generating a citizenship verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct CitizenshipProofGenerationRequest {
    pub merkle_root: u64,
    pub path: u64,
    pub leaf: u64,
}

// Request payload for generating a college credential verification proof
#[derive(Debug, Serialize, Deserialize)]
pub struct CollegeCredentialProofGenerationRequest {
    pub university_public_key: u64,
    pub credential: u64,
    pub signature: u64,
}
