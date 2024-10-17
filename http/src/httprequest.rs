use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Method::Get,
            "post" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "http/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

impl From<&str> for HttpRequest {
    fn from(s: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "Get".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
}
