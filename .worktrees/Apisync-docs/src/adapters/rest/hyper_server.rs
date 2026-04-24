//! Hyper-based HTTP server adapter (minimal skeleton)

use hyper::body::Body;
use hyper::Request as HyperRequest;
use hyper::Response as HyperResponse;
use hyper::server::Server;
use hyper::service::{service_fn, make_service_fn};
use std::convert::Infallible;
use crate::domain::{Request as ApiRequest, Response as ApiResponse, Endpoint};
use std::sync::Arc;

pub struct HyperServer {
    addr: std::net::SocketAddr,
    endpoint: Arc<dyn Endpoint>,
}

impl HyperServer {
    pub fn new(addr: std::net::SocketAddr, endpoint: Arc<dyn Endpoint>) -> Self {
        Self { addr, endpoint }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let make_svc = make_service_fn(move |_| {
            let endpoint = self.endpoint.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: HyperRequest<Body>| {
                    let endpoint = endpoint.clone();
                    async move {
                        // TODO: translate HyperRequest -> ApiRequest, call endpoint.handle
                        // For now, respond 501
                        let res = HyperResponse::builder().status(501).body(Body::from("Not Implemented")).unwrap();
                        Ok::<_, Infallible>(res)
                    }
                }))
            }
        });

        let server = Server::bind(&self.addr).serve(make_svc);
        server.await?;
        Ok(())
    }
}
