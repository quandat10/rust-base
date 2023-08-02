use std::sync::Arc;

use axum::{routing::get, Router};
use http::header;
use tower_http::{
    compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer, trace,
};

use crate::infrastructure::AppState;

use self::health_check::health_checker_handler;

pub mod health_check;

pub struct Cat {
    pub name: String,
}
impl Cat {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub fn router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(Router::new().nest(
            "/api",
            // All public v1 routes will be nested here.
            Router::new().merge(Router::new().route("/health-check", get(health_checker_handler))),
        ))
        .with_state(app_state)
        // High level logging of requests and responses
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        // Mark the `Authorization` request header as sensitive so it doesn't
        // show in logs.
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        // Compress responses
        .layer(CompressionLayer::new())
    // Propagate `X-Request-Id`s from requests to responses
}
