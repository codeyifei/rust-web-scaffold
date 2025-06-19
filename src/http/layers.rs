use crate::http::headers::X_REQUEST_ID;
use axum::http::{Method, Request};
use tower::ServiceBuilder;
use tower::layer::util::{Identity, Stack};
use tower_http::cors::{Any, CorsLayer};
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::request_id::{
    MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct UuidRequestIdGenerator;

impl MakeRequestId for UuidRequestIdGenerator {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        Some(RequestId::new(Uuid::new_v4().to_string().parse().unwrap()))
    }
}

type DefaultRequestIdLayer = ServiceBuilder<
    Stack<
        PropagateHeaderLayer,
        Stack<PropagateRequestIdLayer, Stack<SetRequestIdLayer<UuidRequestIdGenerator>, Identity>>,
    >,
>;
pub fn default_request_id_layer() -> DefaultRequestIdLayer {
    let layer = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            X_REQUEST_ID.clone(),
            UuidRequestIdGenerator::default(),
        ))
        .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
        .layer(PropagateHeaderLayer::new(X_REQUEST_ID.clone()));
    layer
}

pub fn default_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(Any)
}
