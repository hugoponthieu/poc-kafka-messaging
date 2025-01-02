use std::sync::Arc;

use messages_repo::MessagesRepoTrait;
use tokio::sync::RwLock;

use crate::services::kafka_client;

pub mod messages_repo;

#[derive(Clone)]
pub struct Repositories {
    pub messages_repo: Arc<RwLock<messages_repo::MessagesRepo>>,
}

pub trait RepositoriesTrait: Send + Sync {
    fn build(kafka_client: kafka_client::KafkaClient) -> Repositories;
}

impl RepositoriesTrait for Repositories {
    fn build(kafka_client: kafka_client::KafkaClient) -> Repositories {
        let messages_repo = messages_repo::MessagesRepo::build(kafka_client);
        Repositories {
            messages_repo: Arc::new(RwLock::new(messages_repo)),
        }
    }
}
