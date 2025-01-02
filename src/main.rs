use tracing_subscriber;
mod messaging;
mod routes;
mod config;
mod server; 

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = config::AppConfig::make();
    let routes = routes::create_route();
    match server::serve_app(app_config.http_host, app_config.http_port, routes).await {
        Ok(_) => {
            tracing::info!("Server started successfully");
        }
        Err(e) => {
            tracing::error!("Server failed to start: {}", e);
        }
    }
}
