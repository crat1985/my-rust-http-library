// #[derive(Debug)]
// pub enum Body {
//     String(String),
//     Bytes(Vec<u8>),
// }

use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

use crate::{error::HttpError, response::IntoResponse};

pub trait BodyTrait {
    type Error;

    fn parse_request(
        buf: &mut BufReader<&mut TcpStream>,
        content_length_header: usize,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: IntoResponse;
}

impl BodyTrait for Vec<u8> {
    type Error = HttpError;

    fn parse_request(
        buf: &mut BufReader<&mut TcpStream>,
        content_length_header: usize,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: IntoResponse,
    {
        let mut body: Vec<u8> = vec![0; content_length_header];
        if let Err(e) = buf.read_exact(&mut body) {
            return Err(HttpError::InvalidBytesBody(e));
        }
        Ok(body)
    }
}

impl BodyTrait for String {
    type Error = HttpError;

    fn parse_request(
        buf: &mut BufReader<&mut TcpStream>,
        content_length_header: usize,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: IntoResponse,
    {
        let body = Vec::<u8>::parse_request(buf, content_length_header)?;
        String::from_utf8(body).map_err(|e| HttpError::InvalidStringBody(e))
    }
}

impl BodyTrait for () {
    type Error = HttpError;

    fn parse_request(_: &mut BufReader<&mut TcpStream>, _: usize) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Error: IntoResponse,
    {
        Ok(())
    }
}

/// The body of the request
#[derive(Debug)]
pub enum Body {
    None,
    Bytes(Vec<u8>),
    String(String),
}
