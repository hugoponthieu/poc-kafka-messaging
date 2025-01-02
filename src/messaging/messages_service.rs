use std::fmt::Error;

use axum::response::sse::Event;
use futures::Stream;
use tokio_stream::StreamExt;

use crate::repositories::{messages_repo::MessagesRepoTrait, Repositories};

pub async fn stream_kafka_events_rdkafka(
    state: Repositories,
) -> impl Stream<Item = Result<Event, Error>> {
    let messages_repo = state.messages_repo.clone();
    let repo = messages_repo.read().await;

    // Get the stream and map it to Events
    repo.get_topic_message_stream()
        .map(|result| result.map(|msg| Event::default().data(msg)))
}
