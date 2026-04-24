//! Prometheus Adapter

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::application::{MetricExporter, PrometheusExporter};
use crate::domain::Registry;

/// Start Prometheus metrics endpoint
pub async fn start_prometheus_server(
    addr: SocketAddr,
    registry: Registry,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let registry = std::sync::Arc::new(registry);

    let listener = TcpListener::bind(addr).await?;
    println!("Prometheus metrics server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let registry = registry.clone();
                let exporter = PrometheusExporter::new();

                tokio::spawn(async move {
                    let io = TokioIo::new(stream);
                    let service = service_fn(move |_req: Request<hyper::body::Incoming>| {
                        let registry = registry.clone();
                        let exporter = exporter.clone();
                        async move {
                            let metrics = exporter.export(&registry).unwrap_or_default();
                            Ok::<_, std::convert::Infallible>(
                                Response::builder()
                                    .header("content-type", "text/plain; version=0.0.4")
                                    .body(Full::new(Bytes::from(metrics)))
                                    .unwrap(),
                            )
                        }
                    });

                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service)
                        .await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
