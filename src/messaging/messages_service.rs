use axum::response::sse::Event;
use tokio::{sync::mpsc::UnboundedSender, task::AbortHandle};
use tokio_stream::StreamExt;

use crate::repositories::{messages_repo::MessagesRepoTrait, Repositories};

pub async fn stream_kafka_events_rdkafka(
    state: Repositories,
    message_input: UnboundedSender<Event>,
) -> AbortHandle {
    tracing::info!("Getting repo");
    let repo = state.messages_repo.read().await;
    tracing::info!("Got repo");
    // Get the stream and map it to Events
    let (mut raw_stream, handle_streaming_end) = repo.get_topic_message_stream();
    tracing::info!("Got stream");
    tokio::spawn(async move {
        tracing::info!("Starting to stream messages sse");
        while let Some(message) = raw_stream.next().await {
            tracing::info!("Received message");
            match message {
                Ok(msg) => {
                    tracing::info!("Sending message to SSE");
                    let _ = message_input.send(Event::default().data(msg));
                }
                Err(e) => {
                    tracing::error!("Failed to receive message: {}", e);
                    return;
                }
            }
        }
    });
    
    handle_streaming_end
}
