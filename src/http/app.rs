use crate::http::config::Config;
use crate::http::layers::{default_cors_layer, default_request_id_layer};
use crate::result::error::AppError;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::Router;
use std::net::SocketAddr;
use std::ops::Deref;
use std::time::Duration;
use log::info;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub struct App(pub IntoMakeServiceWithConnectInfo<Router, SocketAddr>, pub Config);

impl Deref for App {
    type Target = IntoMakeServiceWithConnectInfo<Router, SocketAddr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl App {
    pub fn new(config: Config, router: Router) -> Self {
        let mut router = router
            .layer(default_request_id_layer())
            .layer(default_cors_layer())
            .layer(TraceLayer::new_for_http());
        if config.timeout > 0 {
            router = router.layer(TimeoutLayer::new(Duration::from_secs(config.timeout)));
        }
        Self(router.into_make_service_with_connect_info::<SocketAddr>(), config)
    }
    
    pub async fn run(self) -> Result<(), AppError> {
        let listener: TcpListener = TcpListener::bind(&self.1.addr).await?;
        info!("Http Server Listening on {}", &self.1.addr);
        axum::serve(listener, self.0).await?;

        Ok(())
    }
}