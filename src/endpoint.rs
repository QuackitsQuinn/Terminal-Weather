use reqwest::{Client, Method, RequestBuilder, Url};

pub struct Endpoint {
    pub url: Url,
    pub query_params: Option<Vec<String>>,
    pub method: Method
}

impl Endpoint {
    /// Create a new endpoint with no query parameters
    pub fn new(url: &str, method: &str) -> Endpoint {
        Endpoint {
            url: Url::parse(url).unwrap(),
            query_params: None,
            method: Method::from_bytes(method.as_bytes()).expect("{} ")
        }
    }
    /// Create a new endpoint with query parameters
    pub fn new_qp(url: &str, method: &str, query_params: Vec<String>) -> Endpoint {
        Endpoint {
            url: Url::parse(url).unwrap(),
            query_params: Some(query_params),
            method: Method::from_bytes(method.as_bytes()).unwrap()
        }
    }
    pub fn builder(&self, client: &Client) -> RequestBuilder {
        let url = self.url.clone();
        let mut strurl  = url.as_str();
        let fmt;
        if let Some(query_params) = &self.query_params {
            println!("{:?}",query_params);
            let query = query_params.join("&");
            println!("{}", query);
            fmt = format!("{}?{}", &strurl, query);
            strurl = fmt.as_str();
        }
        client.request(self.method.clone(), strurl)
    }
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Endpoint {
            url: self.url.clone(),
            query_params: self.query_params.clone(),
            method: self.method.clone()
        }
    }
}
