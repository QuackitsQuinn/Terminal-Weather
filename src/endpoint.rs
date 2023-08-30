use reqwest::{Method, Url};

pub struct Endpoint {
    pub url: Url,
    pub method: Method
}

impl Endpoint {
    pub const fn new(url: &str, method: &str) -> Endpoint {
        Endpoint {
            url: Url::parse(url).unwrap(),
            method: Method::from_bytes(method.as_bytes()).unwrap()
        }
    }
}