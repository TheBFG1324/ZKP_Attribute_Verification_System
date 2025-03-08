use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};

// Testing backend and ZKP_core integrations. Generates and verifies ZK-SNARK proofs via REST API calls. Tests created by OpenAI's o3 model.

#[tokio::test]
async fn test_age_verification_generate() {
    let client = Client::new();
    let request_body = json!({
        "user_age": 25,
        "min_age": 18
    });

    let res = client
        .post("http://localhost:8080/age_verification/generate")
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        res.status().is_success(),
        "Expected success status, got {}",
        res.status()
    );
    let body = res.text().await.expect("Failed to get text");
    println!("Age Verification Generate Response: {}", body);
}

#[tokio::test]
async fn test_citizenship_generate() {
    let client = Client::new();
    let request_body = json!({
        "merkle_root": 10,
        "path": 5,
        "leaf": 5
    });

    let res = client
        .post("http://localhost:8080/citizenship/generate")
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        res.status().is_success(),
        "Expected success status, got {}",
        res.status()
    );
    let body = res.text().await.expect("Failed to get text");
    println!("Citizenship Generate Response: {}", body);
}

#[tokio::test]
async fn test_college_degree_generate() {
    let client = Client::new();
    let request_body = json!({
        "university_public_key": 20,
        "credential": 18,
        "signature": 2
    });

    let res = client
        .post("http://localhost:8080/college_degree/generate")
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        res.status().is_success(),
        "Expected success status, got {}",
        res.status()
    );
    let body = res.text().await.expect("Failed to get text");
    println!("College Credential Generate Response: {}", body);
}

//The following tests check the verification endpoints

#[tokio::test]
async fn test_age_verification_integration() {
    let client = Client::new();

    // First, generate an age verification proof.
    let gen_request = json!({
        "user_age": 25,
        "min_age": 18
    });
    let gen_res = client
        .post("http://localhost:8080/age_verification/generate")
        .json(&gen_request)
        .send()
        .await
        .expect("Failed to send generate request for age verification");
    assert!(
        gen_res.status().is_success(),
        "Generation failed with status: {}",
        gen_res.status()
    );
    let gen_json: serde_json::Value = gen_res
        .json()
        .await
        .expect("Failed to parse generation JSON");
    let proof_str = gen_json["proof"]
        .as_str()
        .expect("Missing 'proof' field")
        .to_string();

    // Wait a moment if needed
    sleep(Duration::from_millis(100)).await;

    // Now verify the proof.
    let verify_request = json!({
        "proof": proof_str,
        "min_age": 18
    });
    let verify_res = client
        .post("http://localhost:8080/age_verification/verify")
        .json(&verify_request)
        .send()
        .await
        .expect("Failed to send verify request for age verification");
    assert!(
        verify_res.status().is_success(),
        "Verification failed with status: {}",
        verify_res.status()
    );
    let verify_body = verify_res
        .text()
        .await
        .expect("Failed to get age verification response text");
    println!("Age Verification Response: {}", verify_body);
}

#[tokio::test]
async fn test_citizenship_integration() {
    let client = Client::new();

    // First, generate a citizenship verification proof.
    let gen_request = json!({
        "merkle_root": 25,
        "path": 10,
        "leaf": 15
    });
    let gen_res = client
        .post("http://localhost:8080/citizenship/generate")
        .json(&gen_request)
        .send()
        .await
        .expect("Failed to send generate request for citizenship");
    assert!(
        gen_res.status().is_success(),
        "Generation failed with status: {}",
        gen_res.status()
    );
    let gen_json: serde_json::Value = gen_res
        .json()
        .await
        .expect("Failed to parse citizenship generation JSON");
    let proof_str = gen_json["proof"]
        .as_str()
        .expect("Missing 'proof' field")
        .to_string();

    sleep(Duration::from_millis(100)).await;

    // Now verify the citizenship proof.
    let verify_request = json!({
        "proof": proof_str,
        "merkle_root": 25
    });
    let verify_res = client
        .post("http://localhost:8080/citizenship/verify")
        .json(&verify_request)
        .send()
        .await
        .expect("Failed to send verify request for citizenship");
    assert!(
        verify_res.status().is_success(),
        "Verification failed with status: {}",
        verify_res.status()
    );
    let verify_body = verify_res
        .text()
        .await
        .expect("Failed to get citizenship verification response text");
    println!("Citizenship Verification Response: {}", verify_body);
}

#[tokio::test]
async fn test_college_credential_integration() {
    let client = Client::new();

    // First, generate a college credential verification proof.
    let gen_request = json!({
        "university_public_key": 20,
        "credential": 18,
        "signature": 2
    });
    let gen_res = client
        .post("http://localhost:8080/college_degree/generate")
        .json(&gen_request)
        .send()
        .await
        .expect("Failed to send generate request for college credential");
    assert!(
        gen_res.status().is_success(),
        "Generation failed with status: {}",
        gen_res.status()
    );
    let gen_json: serde_json::Value = gen_res
        .json()
        .await
        .expect("Failed to parse college credential generation JSON");
    let proof_str = gen_json["proof"]
        .as_str()
        .expect("Missing 'proof' field")
        .to_string();

    sleep(Duration::from_millis(100)).await;

    // Now verify the college credential proof.
    let verify_request = json!({
        "proof": proof_str,
        "university_public_key": 20
    });
    let verify_res = client
        .post("http://localhost:8080/college_degree/verify")
        .json(&verify_request)
        .send()
        .await
        .expect("Failed to send verify request for college credential");
    assert!(
        verify_res.status().is_success(),
        "Verification failed with status: {}",
        verify_res.status()
    );
    let verify_body = verify_res
        .text()
        .await
        .expect("Failed to get college credential verification response text");
    println!("College Credential Verification Response: {}", verify_body);
}
