A tiny HTTP server library without any dependencies.

# Example

```rust
#[derive(Clone)]
pub struct AppState {
    //DB pool, SMTP client...
}

fn main() -> my_http_server_library::Result<()> {
    let app_state = AppState {
        //DB pool, SMTP client...
    };

    let router = Router::with_state(app_state)
        .route("/", get(hello_world_handler));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    my_http_server_library::serve(listener, router)?;

    Ok(())
}

fn hello_world_handler(req: Request) -> Response {
    let content = include_str!("./index.html");
    let app_state = req.state();
    //Do something with app_state
    ResponseBuilder::new()
        .with_body(content, BodyKind::Html)
        .build()
}
```

# Features

- [x] Basic request parsing (HTTP/1.1)
- [ ] Router
  - [x] Basic
  - [x] URL parameters
  - [ ] Path parameters
- [ ] WebSockets
- [ ] CORS
- [ ] TLS support
