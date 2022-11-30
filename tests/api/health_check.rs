use reqwest::Client;

use crate::helpers::spawn_app;

#[tokio::test]
async fn test_health_check_ok() {
    // Spawn the app to send a request to it
    let test_app = spawn_app().await;
    let client = Client::new();
    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Could not send request");

    println!("{:?}", response);

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
