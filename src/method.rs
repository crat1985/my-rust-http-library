#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Options,
    Delete,
}

impl Method {
    pub fn parse(method: &str) -> Self {
        match method.to_lowercase().as_str() {
            "get" => Self::Get,
            "post" => Self::Post,
            "put" => Self::Put,
            "patch" => Self::Patch,
            "options" => Self::Options,
            "delete" => Self::Delete,
            _ => unimplemented!("Method: {method}"),
        }
    }
}
