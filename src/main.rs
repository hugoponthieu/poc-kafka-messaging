use tracing_subscriber;
mod messaging;
mod routes;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let routes = routes::create_route();
    
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}
