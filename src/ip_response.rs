use std::future::Future;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::endpoints;

#[derive(Deserialize, Debug)]
pub struct IpResponse {
    pub status: String,
    pub country: String,
    #[serde(alias="regionName")]
    pub region_name: String,
    pub city: String,
    pub lat: f32,
    pub lon: f32
}

impl IpResponse {
    pub async fn new(client: &Client) -> IpResponse {
        let req_builder = endpoints::get_ip_geolocation_url().builder(client);
        let req = req_builder.build().expect("Building IpResponse request failed!");
        let resp = client.execute(req).await.expect("Response Failed!");
        let txt = resp.text().await.expect("Empty Body!");
        serde_json::from_str(txt.as_str()).expect("Failed Serialization")
    }
}