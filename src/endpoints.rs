use crate::endpoint::Endpoint;


pub fn get_ip_geolocation_url() -> Endpoint {
    Endpoint::new_qp("http://ip-api.com/json", "GET",vec!("fields=49369".to_owned()))
}

pub fn get_weather_url() -> Endpoint {
    Endpoint::new("http://api.openweathermap.org/data/2.5/weather", "GET")
}