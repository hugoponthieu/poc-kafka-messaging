use futures::Stream;
use rdkafka::{consumer::Consumer, Message as _};
use std::{fmt::Error, pin::Pin, sync::Arc};
use tokio::{sync::Mutex, task::AbortHandle};
use tokio_stream::wrappers::UnboundedReceiverStream; // Changed to tokio::sync::Mutex

use crate::services::kafka_client::KafkaClient;

pub struct MessagesRepo {
    pub client: Arc<KafkaClient>,
}

pub trait MessagesRepoTrait: Send + Sync {
    fn build(client: KafkaClient) -> MessagesRepo;
    // Changed return type to be explicit with Pin<Box>
    fn get_topic_message_stream(
        &self,
    ) -> (
        Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>,
        AbortHandle,
    );
}

impl MessagesRepoTrait for MessagesRepo {
    fn build(client: KafkaClient) -> MessagesRepo {
        MessagesRepo {
            client: Arc::new(client),
        }
    }

    fn get_topic_message_stream(
        &self,
    ) -> (
        Pin<Box<dyn Stream<Item = Result<String, Error>> + Send>>,
        AbortHandle,
    ) {
        use tokio_stream::StreamExt;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let tx = Arc::new(Mutex::new(tx));
        let client = self.client.clone();
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
                        match tx.lock().await.send(message_str.to_string()){
                            Ok(()) => {
                                tracing::info!("message sent");
                            }
                            Err(e) => {
                                println!("Failed to receive message: {}", e);
                                return;
                            }
                        };
                        tracing::info!("message sent");
                    }
                    Err(e) => {
                        println!("Failed to receive message: {}", e);
                        return;
                    }
                }
            }
        })
        .abort_handle();
        (
            Box::pin(UnboundedReceiverStream::new(rx).map(|msg: String| Ok(msg))),
            abort_handle,
        )
    }
}
