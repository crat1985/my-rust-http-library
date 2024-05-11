use std::{
    collections::{hash_map::Entry, HashMap},
    io::{BufRead, BufReader, Read},
    net::{SocketAddr, TcpStream},
};

use crate::{error::HttpError, HttpResult};

use super::{http_version::HttpVersion, method::Method};

#[derive(Debug)]
pub struct Request<S: Clone> {
    peer_addr: SocketAddr,
    method: Method,
    uri: String,
    query: HashMap<String, String>,
    http_version: HttpVersion,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
    state: S,
}

impl<S: Clone> Request<S> {
    pub fn parse(stream: &mut TcpStream, state: S) -> HttpResult<Self> {
        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => return Err(HttpError::GetPeerAddrError(e)),
        };

        let mut buf = BufReader::new(stream);

        let (method, uri, http_version) = Self::get_and_parse_request_line(&mut buf);

        let (uri, query) = Self::parse_query_from_uri(&uri)?;

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
                    let mut body: Vec<u8> = vec![0; content_length_header];
                    if let Err(e) = buf.read_exact(&mut body) {
                        return Err(HttpError::InvalidBytesBody(e));
                    }
                    Some(body)
                }
            }
        };

        let req = Self {
            peer_addr,
            method,
            uri: uri.to_string(),
            query,
            http_version,
            headers,
            body,
            state,
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

    /// Parse the URI and returns the URI and the query
    fn parse_query_from_uri(uri: &str) -> HttpResult<(String, HashMap<String, String>)> {
        let (uri, query) = match uri.split_once('?') {
            Some((uri, query)) => {
                let params = query.split('&');
                let mut query = HashMap::new();
                for param in params {
                    match param.split_once('=') {
                        Some((name, value)) => {
                            query.insert(name.to_string(), value.to_string());
                        }
                        None => return Err(HttpError::InvalidQuery),
                    }
                }
                (uri.to_string(), query)
            }
            None => (uri.to_string(), HashMap::new()),
        };
        Ok((uri, query))
    }

    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }

    pub fn http_version(&self) -> &HttpVersion {
        &self.http_version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    ///Consumes the body
    pub fn bytes_body(&mut self) -> Option<Vec<u8>> {
        self.body.take()
    }

    ///Consumes the body
    pub fn string_body(&mut self) -> Option<String> {
        if let Some(body) = self.bytes_body() {
            let body = body.to_vec();
            match String::from_utf8(body) {
                Ok(body) => Some(body),
                Err(e) => {
                    eprintln!("Non-String body : {e}");
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }
}
