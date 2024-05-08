use std::{collections::HashMap, net::TcpStream};

use crate::{response::Response, status_code::StatusCode, ws::WebSocket};

use super::{method::Method, request::Request, response::IntoResponse};

type HandlerFn<S> = Box<dyn Fn(Request<S>) -> Response + Send + Sync>;

pub struct Router<S: Clone> {
    routes: HashMap<Route, HandlerFn<S>>,
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

impl<S: Clone> Router<S> {
    pub fn with_state(state: S) -> Router<S> {
        Router {
            routes: HashMap::new(),
            state,
        }
    }

    pub fn get<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Get], uri, handler)
    }

    pub fn post<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Post], uri, handler)
    }
    pub fn put<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Put], uri, handler)
    }
    pub fn patch<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Patch], uri, handler)
    }
    pub fn options<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Options], uri, handler)
    }
    pub fn delete<E: IntoResponse>(
        self,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        self.several_methods(vec![Method::Delete], uri, handler)
    }

    pub fn several_methods<E: IntoResponse>(
        mut self,
        methods: Vec<Method>,
        uri: &str,
        handler: impl Fn(Request<S>) -> E + Send + Sync + 'static,
    ) -> Self {
        let route = Route {
            methods,
            uri: uri.to_string(),
        };
        let handler = Box::new(move |req| handler(req).into_response());
        self.routes.insert(route, handler);
        self
    }

    pub fn ws_route(mut self, uri: &str, handler: fn(WebSocket)) -> Self {
        let route = Route {
            methods: vec![Method::Get],
            uri: uri.to_string(),
        };
        let handler = Box::new(move |req| {
            let ws = WebSocket::from_request(req).unwrap();
            handler(ws);
            StatusCode::Ok.into_response()
        });
        self.routes.insert(route, handler);
        self
    }

    pub fn handle(&self, req: Request<S>, stream: &mut TcpStream) -> Result<(), StatusCode> {
        let mut matching_routes = self
            .routes
            .iter()
            .filter(|(route, _)| route.uri == req.uri());

        if matching_routes.clone().count() == 0 {
            return Err(StatusCode::NotFound);
        }

        let mut res = match matching_routes.find(|(route, _)| route.methods.contains(req.method()))
        {
            Some((_, handler)) => handler(req),
            None => return Err(StatusCode::MethodNotAllowed),
        };

        res.send_to_stream(stream);

        Ok(())
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Route {
    methods: Vec<Method>,
    uri: String,
}

impl Route {
    pub fn new(methods: Vec<Method>, uri: &str) -> Self {
        Self {
            methods,
            uri: uri.to_string(),
        }
    }
}
