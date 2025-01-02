use std::{convert::Infallible, time::Duration};

use axum::response::{sse::Event, Sse};
use axum_extra::{headers::UserAgent, TypedHeader};
use futures::Stream;

use crate::messaging::messages_service::stream_kafka_events_rdkafka;


pub async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());
    let rx_stream = stream_kafka_events_rdkafka();
    Sse::new(rx_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive-text"),
    )
}
