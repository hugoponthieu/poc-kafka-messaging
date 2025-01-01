use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use axum::response::sse::Event;
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
    Message,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub fn stream_kafka_events_rdkafka() -> impl tokio_stream::Stream<Item = Result<Event, Infallible>>
{
    use tokio_stream::StreamExt;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let tx = Arc::new(Mutex::new(tx));

    tokio::spawn(async move {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", "group")
            .set("bootstrap.servers", "localhost:9092")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&["first-course"])
            .expect("Can't subscribe to specified topic");

        let mut message_stream = consumer.stream();

        while let Some(message) = message_stream.next().await {
            match message {
                Ok(borrowed_message) => {
                    let payload = borrowed_message.payload().unwrap_or(&[]);
                    let message_str = String::from_utf8_lossy(payload);
                    tracing::info!(%message_str);
                    if let Err(e) = tx.lock().unwrap().send(message_str.to_string()) {
                        println!("Failed to send message: {}", e);
                        return;
                    }
                    tracing::info!("message sent");
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    return;
                }
            }
        }
    });

    UnboundedReceiverStream::new(rx).map(|msg: String| {
        Ok(Event::default().data(msg)) // Map message into SSE Event
    })
}
