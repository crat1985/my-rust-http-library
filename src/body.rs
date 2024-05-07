#[derive(Debug)]
pub enum Body {
    String(String),
    Bytes(Vec<u8>),
}
