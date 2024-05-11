use std::{collections::HashMap, sync::Arc};

use crate::{
    response::{IntoResponse, Response},
    route_path::Node,
    status_code::StatusCode,
};

use super::{method::Method, request::Request};

pub(crate) type HandlerFn<S> = Arc<dyn Fn(Request<S>) -> Response + Send + Sync>;

pub struct Router<S: Clone> {
    routes: HashMap<Method, Node<S>>,
    state: S,
}

impl Router<()> {
    pub fn new() -> Router<()> {
        Router {
            routes: HashMap::new(),
            state: (),
        }
    }
}

impl<S: Clone + 'static> Router<S> {
    pub fn with_state(state: S) -> Router<S> {
        Router {
            routes: HashMap::new(),
            state,
        }
    }

    pub fn get<E: IntoResponse + 'static>(self, uri: &str, handler: fn(Request<S>) -> E) -> Self {
        self.insert(Method::Get, uri, handler)
    }

    pub fn post<E: IntoResponse + 'static>(self, uri: &str, handler: fn(Request<S>) -> E) -> Self {
        self.insert(Method::Post, uri, handler)
    }
    pub fn put<E: IntoResponse + 'static>(self, uri: &str, handler: fn(Request<S>) -> E) -> Self {
        self.insert(Method::Put, uri, handler)
    }
    pub fn patch<E: IntoResponse + 'static>(self, uri: &str, handler: fn(Request<S>) -> E) -> Self {
        self.insert(Method::Patch, uri, handler)
    }
    pub fn options<E: IntoResponse + 'static>(
        self,
        uri: &str,
        handler: fn(Request<S>) -> E,
    ) -> Self {
        self.insert(Method::Options, uri, handler)
    }
    pub fn delete<E: IntoResponse + 'static>(
        self,
        uri: &str,
        handler: fn(Request<S>) -> E,
    ) -> Self {
        self.insert(Method::Delete, uri, handler)
    }

    pub fn insert<E: IntoResponse + 'static>(
        mut self,
        method: Method,
        uri: &str,
        handler: fn(Request<S>) -> E,
    ) -> Self {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        let handler = Arc::new(move |req| handler(req).into_response());
        node.insert(uri, handler);

        self
    }

    // pub fn ws_route(mut self, uri: &str, handler: fn(WebSocket)) -> Self {
    //     let route = Route {
    //         methods: vec![Method::Get],
    //         uri: uri.to_string(),
    //     };
    //     let handler = Box::new(move |req| {
    //         let ws = WebSocket::from_request(req).unwrap();
    //         handler(ws);
    //         StatusCode::Ok.into_response()
    //     });
    //     self.routes.insert(route, handler);
    //     self
    // }

    pub fn handle(&self, req: Request<S>) -> Response {
        if let Some(node) = self.routes.get(req.method()) {
            if let Some(handler) = node.get(req.uri()) {
                handler(req)
            } else {
                StatusCode::NotFound.into_response()
            }
        } else {
            StatusCode::NotFound.into_response()
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}
