use newsletter::init;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (app, listener) = init("127.0.0.1:8000").await?;

    axum::serve(listener, app).await
}
