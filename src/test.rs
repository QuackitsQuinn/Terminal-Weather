
#[cfg(test)]
mod req_tests {
    use crate::endpoints;
    #[tokio::test]
    async fn test_ip_resp() {
        let client = reqwest::Client::new();
        let req = endpoints::get_ip_geolocation_url().builder(&client).build().expect("Could not build request");
        println!("{:#?}", req);
        let resp = client.execute(req).await;
        let response = resp.expect("Response Failed");
        println!("{:#?}", response.bytes().await);
    }
}