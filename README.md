A tiny HTTP server library without any dependencies.

Example :

```rust
fn main() -> my_http_server_library::Result<()> {
    let router = Router::with_state(truc)
        .route("/", get(hello_world_handler));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    my_http_server_library::serve(listener, router)?;

    Ok(())
}

fn hello_world_handler(req: Request, state: Arc<&str>) -> Response {
    let content = include_str!("./index.html");
    ResponseBuilder::new()
        .with_body(content, BodyKind::Html)
        .build()
}
```
