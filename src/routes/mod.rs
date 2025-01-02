use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::messaging::messages_controller::sse_handler;


pub fn create_route() -> Router {
    Router::new()
        .route("/sse", get(sse_handler))
        .route("/", get(health_handler))
}

pub async fn health_handler() -> impl IntoResponse {
    "OK"
}

