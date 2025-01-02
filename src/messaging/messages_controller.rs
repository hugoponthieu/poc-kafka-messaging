use std::fmt::Error;
use axum::{extract::State, response::{sse::Event, Sse}};
use futures::Stream;

use crate::{messaging::messages_service::stream_kafka_events_rdkafka, repositories::Repositories};


pub async fn sse_handler(
    State(state): State<Repositories>,
) -> Sse<impl Stream<Item = Result<Event, Error>>> {
    let rx_stream = stream_kafka_events_rdkafka(state).await;
    Sse::new(rx_stream)
}
