use serde::{Serialize, Deserialize};

// Represents a user in the ZKP Identity Verification System.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,                              // Unique user identifier
    pub name: String,                            // User's full name
    pub request: String                          // User's request to either generate, or validate a proof
    pub age: Option<u32>,                        // The user's age (optional)
    pub college_degree_verified: Option<bool>,   // Verification status for the college credential. (optional)
    pub citizenship_verified: Option<bool>,      // Verification status for citizenship. (optional)
    pub proof: Option<String>,                   // A proof converted into a string for transport (optional)
    pub proof_type: Option<String>,              // A proofs type e.g (age_verification, citizenship_verification, or college_credential_verification)
}