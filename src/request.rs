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
            buf.read_line(&mut line).unwrap();
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
                return Err(HttpError::ContentTypeMissing);
            }

            match headers.entry("content-length".to_string()) {
                Entry::Occupied(header) => {
                    let header = header.get();
                    let content_length_header = match header.parse::<usize>() {
                        Ok(length) => length,
                        Err(e) => {
                            return Err(HttpError::InvalidLength(e));
                        }
                    };

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

                Entry::Vacant(_) => {
                    return Err(HttpError::LengthMissing);
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
}
