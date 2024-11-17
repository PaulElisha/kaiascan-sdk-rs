# KaiaScan SDK
A Rust SDK for interacting with the Klaytn blockchain through KaiaScan's API.
Getting Started

1. Obtain API 

-Visit KaiaScan
-Create an account or sign in
-Navigate to API section in your profile
-Generate a new API key for mainnet access
-Copy your Bearer token (format:edcdd09d-XXXX-XXXX-XXXX-XXXXXXXXXXXX)
2. ## Create new Rust project
cargo new kaia_scan_project
`cd kaia_scan_project`

## Add dependencies to Cargo.toml
```
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```
3. ## Configure SDK AUTH And Run the code
`cargo run --example basic`

## Test specific endpoints
`cargo test --test integration_tests`

##  Query token information
`let token = client.get_fungible_token("0x5c74070fdea071359b86082bd9f9b3deaafbe32b").await?;`

// Get NFT details
`let nft = client.get_nft_item(nft_address, token_id).await?;`

## Support
For API-related issues:

KaiaScan API Documentation: https://mainnet-api.kaiascan.io/
Klaytn Documentation: https://docs.klaytn.foundation/

For SDK issues:

Open an issue on GitHub
Check existing issues for solutions

## Legal
Ensure compliance with KaiaScan's Terms of Service when using the API