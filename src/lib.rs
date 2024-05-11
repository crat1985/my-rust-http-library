pub mod error;
pub mod header;
pub mod http_version;
pub mod method;
pub mod request;
pub mod response;
pub mod route_path;
pub mod router;
pub mod status_code;
pub mod ws;

use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{request::Request, response::IntoResponse};

use self::router::Router;

pub type Result<T> = std::result::Result<T, error::Error>;
pub type HttpResult<T> = std::result::Result<T, error::HttpError>;

pub fn serve<S: Clone + Send + Sync + 'static>(
    listener: TcpListener,
    router: Router<S>,
) -> Result<()> {
    let router = Arc::new(router);
    let mut threads = Vec::new();
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => return Err(error::Error::TcpStreamError(e)),
        };
        let thread = smol::spawn(handle_client(stream, router.clone()));
        threads.push(thread);
    }

    smol::block_on(async {
        for thread in threads {
            thread.detach();
        }
    });

    Ok(())
}

async fn handle_client<S: Clone + Send + Sync + 'static>(
    mut stream: TcpStream,
    router: Arc<Router<S>>,
) {
    let req = match Request::parse(&mut stream, router.state().clone()) {
        Ok(req) => req,
        Err(e) => {
            e.into_response().send_to_stream(&mut stream);
            return;
        }
    };

    let mut res = router.handle(req);

    res.send_to_stream(&mut stream);
}

#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, BufReader, Write},
        net::{TcpListener, TcpStream},
        thread,
    };

    use crate::router::Router;

    #[test]
    fn test_app() {
        let router = Router::new().get("/", move |req| "slt");

        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

        thread::spawn(move || {
            crate::serve(listener, router).unwrap();
        });

        let mut connection = TcpStream::connect("127.0.0.1:8080").unwrap();
        let req = r#"GET / HTTP/1.1"#;
        connection.write(req.as_bytes()).unwrap();
        let mut buf = BufReader::new(&mut connection);
        let mut result = String::new();
        buf.read_line(&mut result).unwrap();
        println!("{result}");
        assert_eq!(r#"HTTP/1.1 200 OK"#, result.trim());

        let req = r#"GET /a HTTP/1.1"#;
        connection.write(req.as_bytes()).unwrap();
        let mut buf = BufReader::new(&mut connection);
        let mut result = String::new();
        buf.read_line(&mut result).unwrap();
        assert_eq!(r#"HTTP/1.1 404 NOT FOUND"#, result.trim());

        let req = r#"POST / HTTP/1.1"#;
        connection.write(req.as_bytes()).unwrap();
        let mut buf = BufReader::new(&mut connection);
        let mut result = String::new();
        buf.read_line(&mut result).unwrap();
        assert_eq!(r#"HTTP/1.1 405 METHOD NOT ALLOWED"#, result.trim());
    }
}
