use axum::response::sse::Event;
use tokio::{sync::mpsc::UnboundedSender, task::AbortHandle};
use tokio_stream::StreamExt;

use crate::repositories::{messages_repo::MessagesRepoTrait, Repositories};

pub async fn stream_kafka_events_rdkafka(
    state: Repositories,
    message_input: UnboundedSender<Event>,
) -> AbortHandle {
    let repo = state.messages_repo.read().await;
    // Get the stream and map it to Events
    let (mut raw_stream, handle_streaming_end) = repo.get_topic_message_stream();

    tokio::spawn(async move {
        while let Some(message) = raw_stream.next().await {
            match message {
                Ok(msg) => {
                    let _ = message_input.send(Event::default().data(msg));
                }
                Err(e) => {
                    println!("Failed to receive message: {}", e);
                    return;
                }
            }
        }
    });

    handle_streaming_end
}
