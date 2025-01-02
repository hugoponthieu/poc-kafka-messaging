use axum::{routing::get, Router};

use crate::{messaging::messages_controller::sse_handler, repositories::Repositories};

pub fn create_route(repos: Repositories) -> Router {
    Router::new()
        .route("/sse", get(sse_handler))
        .with_state(repos)
}
