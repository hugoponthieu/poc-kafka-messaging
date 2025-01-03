use std::sync::Arc;

use messages_repo::MessagesRepoTrait;
use tokio::sync::RwLock;

use crate::services::{
    kafka_client,
    mongodb_client::{self, MongoDBClient},
    Services,
};

pub mod messages_repo;

#[derive(Clone)]
pub struct Repositories {
    pub messages_repo: Arc<RwLock<messages_repo::MessagesRepo>>,
}

pub trait RepositoriesTrait: Send + Sync {
    fn build(services: Services) -> Repositories;
}

impl RepositoriesTrait for Repositories {
    fn build(services: Services) -> Repositories {
        let messages_repo = messages_repo::MessagesRepo::build(services);
        Repositories {
            messages_repo: Arc::new(RwLock::new(messages_repo)),
        }
    }
}
