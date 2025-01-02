use kafka_client::KafkaClientTrait;
pub mod kafka_client;


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
}

impl Services {
    pub fn build(kafka_host: &str) -> Services {
        let kafka_client = kafka_client::KafkaClient::build(kafka_host);
        Services { kafka_client }
    }
}