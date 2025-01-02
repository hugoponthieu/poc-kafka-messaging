use rdkafka::ClientConfig;

pub struct KafkaClient {
    pub host: String,
    pub producer: rdkafka::producer::FutureProducer,
    pub consumer: rdkafka::consumer::StreamConsumer,
}

pub trait KafkaClientTrait: Send + Sync {
    fn build(host: &str) -> KafkaClient;
}

impl KafkaClientTrait for KafkaClient {
    fn build(host: &str) -> KafkaClient {
        let mut client_instance = rdkafka::config::ClientConfig::new();
        let client: &mut ClientConfig = client_instance
            .set("bootstrap.servers", host)
            .set("group.id", "my-group")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true");

        let producer: rdkafka::producer::FutureProducer =
            client.create().expect("Producer creation error");

        let consumer: rdkafka::consumer::StreamConsumer =
            client.create().expect("Consumer creation error");

        KafkaClient {
            host: host.to_string(),
            producer,
            consumer,
        }
    }
}
