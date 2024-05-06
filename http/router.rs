use std::{collections::HashMap, io::Write, net::TcpStream};

use super::{
    method::Method,
    request::Request,
    response::{IntoResponse, Response},
};

pub type HandlerFn<E: IntoResponse> = fn(req: Request) -> Result<Response, E>;

pub struct Router<E: IntoResponse> {
    routes: HashMap<Route, HandlerFn<E>>,
}

const NOT_FOUND: &str = concat!(
    "HTTP/1.1 404 Not Found\r\n\r\n",
    include_str!("../../html/404.html")
);

impl<E: IntoResponse> Router<E> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    pub fn route(mut self, route: Route, handler: HandlerFn<E>) -> Self {
        self.routes.insert(route, handler);
        self
    }
    pub fn handle(&self, req: Request, stream: &mut TcpStream) {
        let route = Route::new(req.method.clone(), &req.uri);
        let handler = match self.routes.get(&route) {
            Some(handler) => handler,
            None => {
                stream.write(NOT_FOUND.as_bytes()).unwrap();
                return;
            }
        };
        let _ = route;
        let mut res = match handler(req) {
            Ok(res) => res,
            Err(e) => e.into_response(),
        };

        res.send_to_stream(stream);
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Route {
    method: Method,
    uri: String,
}

impl Route {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
        }
    }
}
