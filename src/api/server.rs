use crate::items;
use crate::tabs;

use axum::{routing::post, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

use super::arguments::AppArguments;

pub async fn serve(args: AppArguments) {
    let app = Router::new()
        .route(
            "/v1/@layers:payments:Tabs:getRelated",
            post(tabs::routes::get),
        )
        .route(
            "/v1/@layers:payments:Items:getRelated",
            post(items::routes::get),
        )
        .layer(TraceLayer::new_for_http());
    info!("Starting server at port {}", args.port);
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
