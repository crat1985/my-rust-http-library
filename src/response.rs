use std::{collections::HashMap, convert::Infallible, io::Write, net::TcpStream};

use super::{header::Header, http_version::HttpVersion, status_code::StatusCode};

#[derive(Debug)]
pub struct Response {
    pub http_version: HttpVersion,
    pub status_code: StatusCode,
    pub headers: HashMap<Header, String>,
    pub body: Option<String>,
}

impl Response {
    pub fn send_to_stream(&mut self, stream: &mut TcpStream) {
        let content_length = if let Some(ref body) = self.body {
            body.len()
        } else {
            0
        };

        self.headers
            .insert(Header::ContentLength, content_length.to_string());

        println!("Response : {self:#?}");

        let mut final_res = format!("{} {}\r\n", self.http_version, self.status_code);

        for (name, value) in &self.headers {
            final_res += &format!("{}: {}\r\n", name, value);
        }

        final_res += "\r\n";

        if let Some(ref body) = self.body {
            final_res += body;
        }

        stream.write(final_res.as_bytes()).unwrap();
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        ResponseBuilder::new().with_status_code(self).build()
    }
}

impl<E: IntoResponse> IntoResponse for Result<E, Infallible> {
    fn into_response(self) -> Response {
        self.unwrap().into_response()
    }
}

impl<E: IntoResponse, F: IntoResponse> IntoResponse for Result<E, F> {
    fn into_response(self) -> Response {
        match self {
            Ok(res) => res.into_response(),
            Err(e) => e.into_response(),
        }
    }
}

pub struct ResponseBuilder {
    http_version: Option<HttpVersion>,
    status_code: Option<StatusCode>,
    headers: HashMap<Header, String>,
    body: Option<String>,
}

pub enum BodyKind {
    Json,
    Html,
    Text,
}

impl BodyKind {
    pub fn content_type(&self) -> &str {
        match self {
            BodyKind::Html => "text/html",
            BodyKind::Json => "application/json",
            BodyKind::Text => "text/plain",
        }
    }
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            http_version: None,
            status_code: None,
            headers: HashMap::new(),
            body: None,
        }
    }
    pub fn with_http_version(mut self, http_version: HttpVersion) -> Self {
        self.http_version = Some(http_version);
        self
    }
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn append_header(mut self, name: Header, value: &str) -> Self {
        self.headers.insert(name, value.to_string());
        self
    }
    pub fn with_body(mut self, body: &str, kind: BodyKind) -> Self {
        self.body = Some(body.to_string());
        self.headers
            .insert(Header::ContentType, kind.content_type().to_string());
        self
    }
    pub fn build(self) -> Response {
        Response {
            http_version: self.http_version.unwrap_or(HttpVersion::HTTP1_1),
            status_code: self.status_code.unwrap_or(StatusCode::Ok),
            headers: self.headers,
            body: self.body,
        }
    }
}
