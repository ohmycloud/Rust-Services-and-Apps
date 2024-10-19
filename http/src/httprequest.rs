use crate::parse::parse_http_request;
use crate::parse::HttpRequest as HttpReq;
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

impl From<String> for HttpRequest {
    fn from(s: String) -> Self {
        let mut s = &s;
        let request: HttpReq = parse_http_request(&mut &***&mut s).unwrap();

        Self {
            method: request.request_line.method.into(),
            version: request.request_line.version.into(),
            resource: Resource::Path(request.request_line.path.into()),
            headers: request.headers.iter().map(|x| {
                (x.key.to_string(), x.value.to_string())
            }).collect(),
            msg_body: request.body.unwrap().to_string(),
        }
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

    fn test_read_http() {
        let s = String::from("GET /greeting HTTP/1.1\r\nHost:
          localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept:
          */*\r\n\r\n");
        let req: HttpRequest =  s.into();
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resource::Path("/greeting".to_string()));

        let mut headers = HashMap::new();
        headers.insert("Host".into(), "localhost:3000".into());
        headers.insert("Accept".into(), "*/*".into());
        headers.insert("User-Agent".into(), "curl/7/64/1".into());
        assert_eq!(req.headers, headers);
    }
}
