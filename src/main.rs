use config::AppConfigTrait;
use repositories::RepositoriesTrait;
use services::ServicesTrait;
use tracing_subscriber;
mod config;
mod messaging;
mod repositories;
mod routes;
mod server;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let app_config = config::AppConfig::make();
    let services =
        services::Services::build(&app_config.kafka_host, &app_config.mongodb_host).await?;
    let repos = repositories::Repositories::build(services.kafka_client);
    let routes = routes::create_route(repos);
    server::serve_app(app_config.http_host, app_config.http_port, routes).await?;
    Ok(())
}
