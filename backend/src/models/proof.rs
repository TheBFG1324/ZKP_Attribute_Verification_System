use serde::{Serialize, Deserialize};

/// A simplified, transport-friendly representation of a ZKP proof.
#[derive(Debug, Serialize, Deserialize)]
pub struct Proof {
    pub proof: String,          // The encoded proof (e.g. Base64 string)
    pub public_input: String,   // The encoded public input associated with the proof.
}