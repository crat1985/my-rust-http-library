pub mod body;
pub mod error;
pub mod header;
pub mod http_version;
pub mod method;
pub mod request;
pub mod response;
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
        let thread = std::thread::spawn({
            let router = router.clone();
            move || {
                handle_client(stream, &router);
            }
        });
        threads.push(thread);
    }
    let _ = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect::<Vec<()>>();
    Ok(())
}

fn handle_client<S: Clone + Send + Sync>(mut stream: TcpStream, router: &Router<S>) {
    let req = match Request::new(&mut stream, router.state().clone()) {
        Ok(req) => req,
        Err(e) => {
            e.into_response().send_to_stream(&mut stream);
            return;
        }
    };

    //println!("Request : {req:?}");

    router.handle(req, &mut stream)
}
