use axum::{Router, http::StatusCode, routing::get};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn init(address: &str) -> Result<(Router, tokio::net::TcpListener), std::io::Error> {
    let app = Router::new().route("/health_check", get(health_check));

    let listener = tokio::net::TcpListener::bind(address).await?;

    Ok((app, listener))
}
