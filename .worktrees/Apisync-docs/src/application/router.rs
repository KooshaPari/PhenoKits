//! Router implementation

use crate::domain::{Request, Response, Endpoint};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router {
    routes: HashMap<String, Arc<dyn Endpoint>>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: HashMap::new() }
    }

    pub fn route<E: Endpoint + 'static>(&mut self, path: impl Into<String>, endpoint: E) {
        self.routes.insert(path.into(), Arc::new(endpoint));
    }

    pub async fn handle(&self, req: Request) -> Response {
        if let Some(ep) = self.routes.get(&req.path) {
            ep.handle(req).await
        } else {
            crate::domain::Response::not_found()
        }
    }
}
