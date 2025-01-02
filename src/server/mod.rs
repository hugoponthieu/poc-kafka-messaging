use axum::Router;

pub async fn serve_app(
    host: String,
    port: u16,
    routes: Router,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, routes).await?;
    Ok(())
}
