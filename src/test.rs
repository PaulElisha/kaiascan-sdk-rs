#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use serde_json::json;

    fn create_mock_response<T>(data: T, code: i32, msg: &str) -> String
    where
        T: Serialize,
    {
        json!({
            "code": code,
            "data": data,
            "msg": msg
        })
        .to_string()
    }

    #[tokio::test]
    async fn test_get_fungible_token() {
        let mock_token = json!({
            "contractType": "ERC20",
            "name": "TestToken",
            "symbol": "TT",
            "icon": "https://example.com/icon.png",
            "decimal": 18,
            "totalSupply": 1000000.0,
            "totalTransfers": 500,
            "officialSite": "https://example.com",
            "burnAmount": 1000.0,
            "totalBurns": 10
        });

        let token_address = "0x1234567890abcdef";
        let mock_path = format!("/api/v1/tokens/{}", token_address);

        let _m = mock("GET", &mock_path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_response(mock_token, 0, "Success"))
            .create();

        // Replace the BASE_URL with the mock server URL for testing
        let mock_base_url = server_url();
        let mut client = KaiaScan::new().unwrap();
        client.set_base_url(&mock_base_url); // Assume a `set_base_url` method exists

        let result = client.get_fungible_token(Address::new(token_address.into())).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "TestToken");
        assert_eq!(response.symbol, "TT");
        assert_eq!(response.total_supply, "1000000.0");
    }
}
