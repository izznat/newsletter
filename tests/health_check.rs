#[tokio::test]
async fn health_check_returns_200_status_code() {
    let address = spawn_app().await.expect("Failed to spawn app.");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute /health_check request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> Result<String, std::io::Error> {
    let (app, listener) = newsletter::init("127.0.0.1:0").await?;

    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{port}");

    tokio::spawn(axum::serve(listener, app).into_future());

    Ok(address)
}
