
#[cfg(test)]
mod req_tests {
    use crate::endpoints;
    use crate::ip_response::IpResponse;

    #[tokio::test]
    async fn test_ip_resp() {
        let client = reqwest::Client::new();
        let ip_loc = IpResponse::new(&client).await;
        println!("{:#?}", ip_loc)
    }
}