use std::{
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::request::Request;

use self::router::Router;

pub mod header;
pub mod http_version;
pub mod method;
pub mod request;
pub mod response;
pub mod router;
pub mod status_code;

pub fn serve<S: Clone + Send + Sync + 'static>(listener: TcpListener, router: Router<S>) {
    let router = Arc::new(router);
    let mut threads = Vec::new();
    for stream in listener.incoming() {
        let thread = std::thread::spawn({
            let router = router.clone();
            move || {
                handle_client(stream.unwrap(), &router);
            }
        });
        threads.push(thread);
    }
    let _ = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect::<Vec<()>>();
}

fn handle_client<S: Clone + Send + Sync>(mut stream: TcpStream, router: &Router<S>) {
    let req = match Request::new(&mut stream) {
        Ok(req) => req,
        Err(mut e) => {
            e.send_to_stream(&mut stream);
            return;
        }
    };

    println!("Request : {req:#?}");

    router.handle(req, &mut stream)
}
