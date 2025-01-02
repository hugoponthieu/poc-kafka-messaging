use axum::{
    extract::State,
    response::{sse::Event, Sse},
};
use futures::stream::StreamExt;
use futures::Stream;
use std::fmt::Error; // Make sure to import this

use crate::{messaging::messages_service::stream_kafka_events_rdkafka, repositories::Repositories};

pub async fn sse_handler(
    State(state): State<Repositories>,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    let (sse_input, sse_event_stream) = tokio::sync::mpsc::unbounded_channel::<Event>();

    let abort_handle = stream_kafka_events_rdkafka(state, sse_input.clone()).await;
    tokio::spawn(async move {
        sse_input.closed().await;
        abort_handle.abort();
        tracing::info!("sse_handler aborted");
    });
    let stream = tokio_stream::wrappers::UnboundedReceiverStream::new(sse_event_stream).map(Ok);

    Sse::new(stream)
}
