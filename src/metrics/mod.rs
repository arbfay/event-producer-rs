use log::info;
use prometheus::{Registry, Encoder};
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use std::sync::Arc;
use tokio::runtime::Runtime;

use hyper::server::Server;
use hyper::{Body, Request, Response};
use routerify::{Router, RouterService, ext::RequestExt};
use std::{convert::Infallible};

use crate::settings::types::MetricsSettings;

async fn get_metrics(
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let registry = req.data::<Arc<Box<Registry>>>().unwrap();
    let metrics = registry.gather();
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();    
    Ok(Response::new(Body::from(buffer)))
}

fn router_service(registry: Arc<Box<Registry>>) -> Router<Body, Infallible> {
    Router::builder()
        .data(registry)
        .get("/metrics", get_metrics)
        .build()
        .unwrap()
}

pub fn start_metrics_service(settings: MetricsSettings, registry: Box<Registry>){
    info!("Starting metrics service");
    let registry_ptr = std::sync::Arc::new(registry);
    
    let runner = Runtime::new().expect("Failed to start async runtime for metrics server");
    runner.block_on(async {
        let addr = SocketAddr::new(IpAddr::from(Ipv4Addr::LOCALHOST), settings.port);

        let router = RouterService::new(router_service(registry_ptr.clone())).unwrap();
        let server = Server::bind(&addr).serve(router);
        server.await.unwrap();
    });
}