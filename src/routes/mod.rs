use axum::{
    response::{sse::Event, IntoResponse, Sse},
    routing::get,
    Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use futures::stream::{self, Stream};
use kafka::Error;
use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio_stream::StreamExt as _;

use crate::messaging::{process_by_client, stream_kafka_events, stream_kafka_events_rdkafka};

pub fn create_route() -> Router {
    Router::new()
        .route("/sse", get(sse_handler))
        .route("/", get(health_handler))
}

pub async fn health_handler() -> impl IntoResponse {
    "OK"
}

pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    let rx_stream = stream_kafka_events_rdkafka();
    Sse::new(rx_stream).keep_alive(axum::response::sse::KeepAlive::new()
        .interval(Duration::from_secs(15))
        .text("keep-alive-text"))
}

