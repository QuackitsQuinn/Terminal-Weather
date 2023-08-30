use reqwest::{Method, Request, Url};

pub struct Endpoint {
    pub url: Url,
    pub query_params: Option<Vec<String>>,
    pub method: Method
}

impl Endpoint {
    /// Create a new endpoint with no query parameters
    pub const fn new(url: &str, method: &str) -> Endpoint {
        Endpoint {
            url: Url::parse(url).unwrap(),
            query_params: None,
            method: Method::from_bytes(method.as_bytes()).unwrap()
        }
    }
    /// Create a new endpoint with query parameters
    pub const fn new_qp(url: &str, method: &str, query_params: Vec<String>) -> Endpoint {
        Endpoint {
            url: Url::parse(url).unwrap(),
            query_params: Some(query_params),
            method: Method::from_bytes(method.as_bytes()).unwrap()
        }
    }
    pub fn build(&self) -> Request {
        let mut url = self.url.clone().as_str();
        if let Some(queryParams) = &self.query_params {
            let query = queryParams.join("&");
            url = (format!("{}?{}", url, query).as_str());
        }
        let mut request = Request::new(self.method.clone(), Url::parse(url.clone()).unwrap());
        request
    }
}
