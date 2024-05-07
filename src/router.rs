use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::response::Response;

use super::{method::Method, request::Request, response::IntoResponse};

type HandlerFn<S> = Box<dyn Fn(Request, S) -> Response + Send + Sync>;

pub struct MethodAndHandlerFn<S: Clone>(Method, HandlerFn<S>);

pub fn get<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Get,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub fn post<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Post,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub fn put<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Put,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub fn patch<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Patch,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub fn options<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Options,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub fn delete<E, F, S>(handler: E) -> MethodAndHandlerFn<S>
where
    E: Fn(Request, S) -> F + 'static + Send + Sync,
    F: IntoResponse,
    S: Clone + Send + Sync,
{
    MethodAndHandlerFn(
        Method::Delete,
        Box::new(move |req, state| handler(req, state).into_response()),
    )
}

pub struct Router<S: Clone> {
    routes: HashMap<Route, HandlerFn<S>>,
    state: S,
}

const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

impl<S: Clone> Router<S> {
    pub fn new() -> Router<()> {
        Router {
            routes: HashMap::new(),
            state: (),
        }
    }
    pub fn with_state(state: S) -> Router<S> {
        Router {
            routes: HashMap::new(),
            state,
        }
    }
    pub fn route(
        mut self,
        uri: &str,
        MethodAndHandlerFn(method, handler): MethodAndHandlerFn<S>,
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
        let mut res = handler(req, self.state.clone()).into_response();

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
