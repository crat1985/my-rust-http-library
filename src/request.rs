use std::{
    collections::{hash_map::Entry, HashMap},
    io::{BufRead, BufReader, Read},
    net::{SocketAddr, TcpStream},
};

use crate::{body::Body, error::HttpError, HttpResult};

use super::{http_version::HttpVersion, method::Method};

#[derive(Debug)]
pub struct Request {
    pub peer_addr: SocketAddr,
    pub method: Method,
    pub uri: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> HttpResult<Self> {
        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => return Err(HttpError::GetPeerAddrError(e)),
        };

        let mut buf = BufReader::new(stream);

        let (method, uri, http_version) = Self::get_and_parse_request_line(&mut buf);

        let mut headers = Self::get_and_parse_headers(&mut buf);

        let body = if method == Method::Get {
            None
        } else {
            if !headers.contains_key("content-type") {
                return Err(HttpError::ContentTypeMissing);
            }

            match headers.entry("content-length".to_string()) {
                Entry::Vacant(_) => {
                    return Err(HttpError::LengthMissing);
                }

                Entry::Occupied(header) => {
                    let header = header.get();
                    let content_length_header = match header.parse::<usize>() {
                        Ok(length) => length,
                        Err(e) => {
                            return Err(HttpError::InvalidLength(e));
                        }
                    };
                    Self::get_body(&mut buf, content_length_header)
                }
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

    fn get_and_parse_request_line(
        buf: &mut BufReader<&mut TcpStream>,
    ) -> (Method, String, HttpVersion) {
        let mut request_line = String::new();
        buf.read_line(&mut request_line).unwrap();
        let mut request_line = request_line.trim().splitn(3, " ");
        let method = request_line.next().unwrap();
        let method = Method::parse(method);
        let uri = request_line.next().unwrap();
        let http_version = request_line.next().unwrap();
        let http_version = HttpVersion::parse(http_version).unwrap();
        (method, uri.to_string(), http_version)
    }

    fn get_and_parse_headers(buf: &mut BufReader<&mut TcpStream>) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            buf.read_line(&mut line).unwrap();
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let (header_name, header_value) = line.split_once(": ").unwrap();

            headers.insert(header_name.to_lowercase(), header_value.to_string());
        }
        headers
    }

    fn get_body(buf: &mut BufReader<&mut TcpStream>, content_length_header: usize) -> Option<Body> {
        let mut body = Vec::with_capacity(content_length_header);
        let count = buf.read(&mut body).unwrap();
        if count == 0 {
            None
        } else {
            let body = match String::from_utf8(body.clone()) {
                Ok(body) => Body::String(body),
                Err(_) => Body::Bytes(body),
            };
            Some(body)
        }
    }
}
