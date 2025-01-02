use axum::Router;



pub async fn serve_app(host: String , port: u16, routes: Router) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    let listener = match tokio::net::TcpListener::bind(&addr)
        .await{
            Ok(listener) => listener,
            Err(e) => {
                tracing::error!("Failed to bind to address: {}", e);
                return Err(Box::new(e));
            }
        };
    match axum::serve(listener, routes).await {
        Ok(_) => {
            tracing::info!("Server listening on {}", addr);
        }
        Err(e) => {
            tracing::error!("Server failed: {}", e);
            return Err(Box::new(e));
        }
    } 
    Ok(())
}