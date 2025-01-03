use futures::Stream;
use model::Message;
use mongodb::Collection;
use rdkafka::{consumer::Consumer, Message as _};
use std::{error::Error, pin::Pin, sync::Arc};
use tokio::{sync::Mutex, task::AbortHandle};
use tokio_stream::wrappers::UnboundedReceiverStream; // Changed to tokio::sync::Mutex

use crate::services::{kafka_client::KafkaClient, mongodb_client::MongoDBClient, Services};
mod model;

pub struct MessagesRepo {
    pub kafka_client: Arc<KafkaClient>,
    pub mongodb_client: Arc<MongoDBClient>,
}

pub trait MessagesRepoTrait: Send + Sync {
    fn build(services: Services) -> MessagesRepo;
    // Changed return type to be explicit with Pin<Box>
    fn get_topic_message_stream(
        &self,
    ) -> (
        Pin<Box<dyn Stream<Item = Result<Message, Box<dyn Error>>> + Send>>,
        AbortHandle,
    );

    async fn save_message(&self, message: Message) -> Result<Message, Box<dyn Error>>;
}

impl MessagesRepoTrait for MessagesRepo {
    fn build(services: Services) -> MessagesRepo {
        MessagesRepo {
            kafka_client: Arc::new(services.kafka_client),
            mongodb_client: Arc::new(services.mongodb_client),
        }
    }

    async fn save_message(&self, message: Message) -> Result<Message, Box<dyn Error>> {
        let message_collection: Collection<Message> = self
            .mongodb_client
            .client
            .database("beep")
            .collection("messages");
        let _ = message_collection.insert_one(&message).await?;
        Ok(message)
    }

    fn get_topic_message_stream(
        &self,
    ) -> (
        Pin<Box<dyn Stream<Item = Result<Message, Box<dyn Error>>> + Send>>,
        AbortHandle,
    ) {
        use tokio_stream::StreamExt;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let tx = Arc::new(Mutex::new(tx));
        let client = self.kafka_client.clone();
        let tx = tx.clone();

        let abort_handle = tokio::spawn(async move {
            client
                .consumer
                .subscribe(&["first-course"])
                .expect("Can't subscribe to specified topic");

            let mut message_stream = client.consumer.stream();

            while let Some(message) = message_stream.next().await {
                match message {
                    Ok(borrowed_message) => {
                        let payload = borrowed_message.payload().unwrap_or(&[]);
                        let message_str = String::from_utf8_lossy(payload);
                        tracing::info!(%message_str);
                        match tx.lock().await.send(message_str.to_string()) {
                            Ok(()) => {
                                tracing::info!("message sent");
                            }
                            Err(e) => {
                                tracing::error!("Failed to receive message: {}", e);
                                return;
                            }
                        };
                        tracing::info!("message sent");
                    }
                    Err(e) => {
                        tracing::error!("Failed to receive message: {}", e);
                        return;
                    }
                }
            }
        })
        .abort_handle();
        (
            Box::pin(UnboundedReceiverStream::new(rx).map(|msg: String| {
                Ok(Message {
                    _id: None,
                    content: msg,
                })
            })),
            abort_handle,
        )
    }
}
