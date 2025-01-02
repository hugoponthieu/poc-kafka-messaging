use tracing_subscriber;
use config::AppConfigTrait;
mod config;
mod messaging;
mod routes;
mod server;
mod services;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = config::AppConfig::make();
    let services = services::Services::build(&app_config.kafka_host);
    let routes = routes::create_route();
    server::serve_app(app_config.http_host, app_config.http_port, routes)
    .await
    .expect("Server failed to start");
}
