use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::response::Response;

use super::{method::Method, request::Request, response::IntoResponse};

type HandlerFn = Box<dyn Fn(Request) -> Response + Send + Sync>;

pub struct MethodAndHandlerFn(Method, HandlerFn);

pub fn get<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Get,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub fn post<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Post,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub fn put<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Put,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub fn patch<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Patch,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub fn options<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Options,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub fn delete<E, F>(handler: E) -> MethodAndHandlerFn
where
    E: Fn(Request) -> F + 'static + Send + Sync,
    F: IntoResponse,
{
    MethodAndHandlerFn(
        Method::Delete,
        Box::new(move |req| handler(req).into_response()),
    )
}

pub struct Router {
    routes: HashMap<Route, HandlerFn>,
}

const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    pub fn route(
        mut self,
        uri: &str,
        MethodAndHandlerFn(method, handler): MethodAndHandlerFn,
    ) -> Self {
        let route = Route::new(method, uri);
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
        let mut res = handler(req).into_response();

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
