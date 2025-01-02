use tracing_subscriber;
use repositories::RepositoriesTrait;
use services::ServicesTrait;
use config::AppConfigTrait;
mod config;
mod messaging;
mod routes;
mod server;
mod services;
mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_config = config::AppConfig::make();
    let services = services::Services::build(&app_config.kafka_host);
    let repos = repositories::Repositories::build(services.kafka_client);
    let routes = routes::create_route(repos);
    server::serve_app(app_config.http_host, app_config.http_port, routes)
    .await
    .expect("Server failed to start");
}
