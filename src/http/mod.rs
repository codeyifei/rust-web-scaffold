use crate::http::config::Config;
use crate::http::layers::{default_cors_layer, default_request_id_layer};
use axum::Router;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub mod config;
pub mod headers;
pub mod layers;

pub mod app;

pub fn default_app(cfg: Config, router: Router) -> Router
{
    let router = router
        .layer(default_request_id_layer())
        .layer(default_cors_layer())
        .layer(TraceLayer::new_for_http());
    if cfg.timeout > 0 {
        return router.layer(TimeoutLayer::new(Duration::from_secs(cfg.timeout)));
    }
    router
}
