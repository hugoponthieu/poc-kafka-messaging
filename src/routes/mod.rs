use axum::{
    response::{sse::Event, IntoResponse, Sse},
    routing::get,
    Router,
};
use tokio_stream::StreamExt as _;
use axum_extra::{headers::UserAgent, TypedHeader};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};

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

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
