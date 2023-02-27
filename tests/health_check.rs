use std::net::TcpListener;

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let base_url = spawn_app().await;

    // Use a reqwest client to interact with the api.
    let client = reqwest::Client::new();

    // Check health_check endpoint
    let response = client
        .get(&format!("{base_url}{}", "/health_check"))
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