use std::net::TcpListener;

use reqwest::StatusCode;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let base_url = spawn_app().await;

    // Use a reqwest client to interact with the api.
    let client = reqwest::Client::new();

    // Check health_check endpoint
    let response = client
        .get(&format!("{base_url}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
    .post(&format!("{base_url}/subscriptions",))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

    assert_eq!(StatusCode::OK, response.status());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "the email"),
        ("email=ursula_le_guin@40gmail.com", "the email"),
        ("", "both name and email.")
    ];

    for (invalid_body, missing_fields) in test_cases {
        let response = client
        .post(&format!("{base_url}/subscriptions",))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request");

        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_content_type() {
    let base_url = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
    .post(&format!("{base_url}/subscriptions",))
    .header("Content-Type", "")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request");

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}