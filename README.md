 # ZKP Identity Verification System

 ## Introduction

 The ZKP Identity Verification System is a modular framework designed to enable privacy-preserving verification of identity attributes using zero-knowledge proofs (zk-SNARKs). Built primarily in Rust—with an optional Solidity smart contract layer for on-chain verification—this system allows users to prove specific attributes (such as age, citizenship, or employment status) without exposing sensitive personal data. It offers a scalable and efficient foundation for applications in Web3, digital voting, secure authentication, and more.

 ## Attributes Verification

 This system is capable of verifying the following attributes:

 - College Degree Credential: Validate the possession of a college degree, ensuring authenticity without exposing detailed academic records.
 - Citizenship: Confirm citizenship status using cryptographically secured government-issued data.
 - Age Verification: Prove that the user meets a minimum age requirement (e.g., over 18) without disclosing the exact birthdate.

 Each attribute is implemented as its own zk-SNARK circuit within the system, ensuring that sensitive inputs remain confidential while enabling public verification.

| Circuit                        | Technique Used                  | What is Public?        | What is Private?              |
|--------------------------------|--------------------------------|------------------------|--------------------------------|
| Citizenship Verification       | Merkle Tree Inclusion Proof   | Merkle Root           | User’s Hash & Merkle Path      |
| College Credential Verification | Digital Signature Check       | University Public Key | User’s Credential & Signature |
| Age Verification               | Numeric Comparison            | Minimum Age           | User’s Age                     |

 ## Sample User Workflow

 1. Initiation:
 The user interacts with the system (via a web interface or API client) and selects an attribute to verify (e.g., proving age over 18).

 2. Data Submission:
 The user submits a minimal set of private inputs (such as a commitment to their secret data) along with necessary public parameters (like the minimum age threshold).

 3. Proof Generation:
 The API layer forwards the request to the Rust-based zk-SNARK core. The appropriate circuit processes the input, generating a zero-knowledge proof that attests to the validity of the attribute without exposing the underlying data.

 4. Local Verification:
 The system can perform an immediate, local verification of the generated proof using the included Rust verifier.

 5. On-Chain Verification (Optional):
 If required, the proof can be submitted to a Solidity smart contract for decentralized on-chain verification, adding an extra layer of trust.

 6. Result Communication:
 The verification result is then returned to the user via the API, confirming that the attribute meets the necessary criteria.

 ## Tech Stack

 - Rust:
   Implements the zk-SNARK circuits in zkp_core/ for proof generation and local verification.
   Powers the backend API (backend/) for handling requests and orchestrating the verification flow.
 - Solidity:
   Develops the smart contract layer (contract/) for optional on-chain verification.
 - zk-SNARKs:
   The underlying cryptographic proof system that enables zero-knowledge proofs for privacy-preserving attribute verification.
 - Hardhat:
   Used for smart contract development and deployment configuration (note: only configuration is included, without migrations or tests).

 ## Repository Structure
 ```plaintext
ZKP_Identity_Verification_System/
├── contract/                        # On-chain verification smart contracts
│   ├── src/                         # Solidity contracts for each verifiable attribute
│   │   ├── CollegeDegreeCredential.sol
│   │   ├── Citizenship.sol
│   │   └── AgeVerification.sol
│   └── hardhat.config.js            # Smart contract configuration
│
├── zkp_core/                        # Rust-based zk-SNARK system
│   ├── circuits/                    # zk-SNARK circuits for each attribute
│   │   ├── college_degree_credential/
│   │   │   └── witness_calculator.rs
│   │   ├── citizenship/
│   │   │   └── witness_calculator.rs
│   │   └── age_verification/
│   │       └── witness_calculator.rs
│   ├── src/                         # Core Rust code for proof generation and verification
│   │   ├── proof_system.rs          # Implements proof generation logic
│   │   └── lib.rs                   # Shared library code for the ZKP system
│   └──  Cargo.toml                   # Rust project configuration
│
├── backend/                         # API layer for interacting with the ZKP system
│   ├── src/
│   │   ├── tests/                  # API endpoint tests for ZK-SNARK proof generation and verification
│   │   ├── routes/                  # API endpoints for each verification attribute
│   │   │   ├── college_degree.rs
│   │   │   ├── citizenship.rs
│   │   │   └── age_verification.rs
│   │   ├── models/                  # Data models (e.g., proof structures, user data, and responses)
│   │   │   ├── proof_generation.rs
│   │   │   ├── proof_generation.rs
│   │   │   └── response.rs
│   │   ├── controllers/             # Business logic for handling API requests
│   │   │   ├── proof_generator_controller.rs
│   │   │   └── proof_verifier_controller.rs
│   │   ├── main.rs                  # API entry point
│   └── Cargo.toml                   # Rust project configuration for the backend
│
├── docs/                            # Project documentation
│   ├── architecture.md              # High-level architecture overview and diagrams
│   ├── api_reference.md             # Detailed API endpoint documentation
│   ├── circuits_design.md           # Design rationale and specifics for each circuit
│   ├── user_flow.md                 # Explanation of the user journey through the system
│   └── README.md                    # General documentation overview
│
├── .gitignore                       # Files and directories to ignore in Git
└── README.md                        # Main project overview and setup instructions

```
 ## Getting Started

 1. Clone the Repository:

```bash
git clone https://github.com/yourusername/ZKP_Identity_Verification_System.git
cd ZKP_Identity_Verification_System
```
 2. Set Up the Rust Environment:

 - Install Rust if not already installed.
 - Navigate to each Rust project (zkp_core/ and backend/) and run:

 ```bash
cargo build
``` 
 3. Configure Environment Variables (Backend):

 - Copy .env.example to .env in the backend/ directory and update the configuration as needed.

 4. Run the Backend API:

 ```bash
cd backend
cargo run
```
 5. Deploy & Interact with Smart Contracts:

 - Review contract/hardhat.config.js for configuration details.
 - Use Hardhat commands to deploy your contracts if on-chain verification is required.
