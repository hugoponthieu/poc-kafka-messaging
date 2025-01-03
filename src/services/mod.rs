use std::error::Error;

use kafka_client::KafkaClientTrait;
use mongodb_client::MongoDBClientTrait;
pub mod kafka_client;
pub mod mongodb_client;
/// The `Services` struct encapsulates various service clients, including a Kafka client.
///
/// # Fields
///
/// * `kafka_client` - An instance of `KafkaClient` used to interact with Kafka.
///
/// # Examples
///
/// ```
/// let services = Services::build("localhost:9092");
/// ```
pub struct Services {
    pub kafka_client: kafka_client::KafkaClient,
    pub mongodb_client: mongodb_client::MongoDBClient,
}

pub trait ServicesTrait: Send + Sync {
    async fn build(kafka_host: &str, mongodb_host: &str) -> Result<Services, Box<dyn Error>>;
}

impl ServicesTrait for Services {
    async fn build(kafka_host: &str, mongodb_host: &str) -> Result<Services, Box<dyn Error>> {
        let kafka_client = kafka_client::KafkaClient::build(kafka_host);
        let mongodb_client = mongodb_client::MongoDBClient::build(mongodb_host).await?;
        Ok(Services {
            kafka_client,
            mongodb_client,
        })
    }
}
