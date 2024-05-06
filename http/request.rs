use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::{SocketAddr, TcpStream},
};

use super::{
    http_version::HttpVersion,
    method::Method,
    response::{Response, ResponseBuilder},
    status_code::StatusCode,
};

#[derive(Debug)]
pub struct Request {
    pub peer_addr: SocketAddr,
    pub method: Method,
    pub uri: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Self, Response> {
        let peer_addr = stream.peer_addr().unwrap();

        let mut buf = BufReader::new(stream);

        //Beginning - Method, Uri and Http version
        let mut request_line = String::new();
        buf.read_line(&mut request_line).unwrap();
        let mut request_line = request_line.trim().splitn(3, " ");

        let method = request_line.next().unwrap();
        let method = Method::parse(method);
        let uri = request_line.next().unwrap();
        let http_version = request_line.next().unwrap();
        let http_version = HttpVersion::parse(http_version).unwrap();
        let _ = request_line;
        //End - Method, Uri and Http version

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            buf.read_line(&mut line);
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let (header_name, header_value) = line.split_once(": ").unwrap();

            headers.insert(header_name.to_lowercase(), header_value.to_string());
        }
        let body = if method == Method::Get {
            None
        } else {
            if !headers.contains_key("content-type") {
                let res = ResponseBuilder::new()
                    .with_status_code(StatusCode::Forbidden)
                    .build();
                return Err(res);
            }
            if !headers.contains_key("content-length") {
                let res = ResponseBuilder::new()
                    .with_status_code(StatusCode::Forbidden)
                    .build();
                return Err(res);
            }

            let content_length_header = headers
                .get("content-length")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let mut body = Vec::with_capacity(content_length_header);
            buf.read(&mut body).unwrap();
            if body.is_empty() {
                None
            } else {
                Some(String::from_utf8(body).unwrap())
            }
        };

        let req = Self {
            peer_addr,
            method,
            uri: uri.to_string(),
            http_version,
            headers,
            body,
        };

        Ok(req)
    }
}
