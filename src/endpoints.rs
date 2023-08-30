use crate::endpoint::Endpoint;

const IP_GEOLOCATION_URL: Endpoint =
    Endpoint::new_qp("http://ip-api.com/json", "GET", vec!["fields=57561".to_string()]);