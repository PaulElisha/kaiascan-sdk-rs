mod test;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const BASE_URL: &str = "https://mainnet-oapi.kaiascan.io/";
const AUTH_TOKEN: &'static str = "";
const TOKENS_ENDPOINT: &str = "api/v1/tokens";
const NFTS_ENDPOINT: &str = "api/v1/nfts";

// Custom type for blockchain addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl AsRef<str> for Address {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountKeyHistory {
    pub address: String,
    pub key_type: String,
    pub public_key: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountKeyHistoryResponse {
    pub paging: Paging,
    pub results: Vec<AccountKeyHistory>,
}
#[derive(Debug, Deserialize)]
pub struct KlayPrice {
    pub btc_price: String,
    pub market_cap: String,
    pub total_supply: String,
    pub usd_price: String,
    pub usd_price_changes: String,
    pub volume: String,
}

#[derive(Debug, Deserialize)]
pub struct KaiaSummary {
    pub avg_block_time1h: String,
    pub avg_block_time24h: String,
    pub avg_tx_per_block24h: f64,
    pub consensus_node: i32,
}

#[derive(Debug, Deserialize)]
pub struct KaiaInfoResponse {
    pub klay_price: KlayPrice,
    pub summary: KaiaSummary,
}

#[derive(Debug, Deserialize)]
pub struct BlockRewardRecipient {
    pub address: String,
    pub amount: String,
    pub name: String,
    #[serde(rename = "type")]
    pub reward_type: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockRewardDistribution {
    pub amount: String,
    #[serde(rename = "type")]
    pub distribution_type: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockRewardsResponse {
    pub burnt_fee: String,
    pub distributions: Vec<BlockRewardDistribution>,
    pub minted: String,
    pub recipients: Vec<BlockRewardRecipient>,
    pub total_fee: String,
}

// #[derive(Debug, Deserialize)]
// pub struct BlockBurns {
//     pub block_id: i64,
//     pub amount: String,
//     pub datetime: String,
// }
#[derive(Debug, Deserialize)]
pub struct BurnSummary {
    pub accumulate_burnt: String,
    pub accumulate_burnt_fees: String,
    pub accumulate_burnt_kaia: String,
    pub kip103_burnt: String,
    pub kip160_burnt: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockBurns {
    pub block_id: i64,
    pub amount: String,
    pub datetime: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockRewards {
    pub block_id: i64,
    pub rewards: Vec<BlockRewardDetail>,
}

#[derive(Debug, Deserialize)]
pub struct BlockRewardDetail {
    pub address: String,
    pub amount: String,
    pub reward_type: String,
}

#[derive(Debug, Deserialize)]
pub struct InternalTransaction {
    pub block_id: i64,
    pub transaction_hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub datetime: String,
}

#[derive(Debug, Deserialize)]
pub struct InternalTransactionsResponse {
    pub paging: Paging,
    pub results: Vec<InternalTransaction>,
}

#[derive(Debug, Deserialize)]
pub struct BlockDetails {
    pub block_id: i64,
    pub datetime: String,
    pub hash: String,
    pub total_transaction_count: i64,
    pub block_reward: BlockReward,
}
#[derive(Debug, Deserialize)]
pub struct BlocksResponse {
    pub blocks: Vec<BlockDetails>,
}

#[derive(Debug, Deserialize)]
pub struct BlockReward {
    pub minted: String,
    pub total_fee: String,
    pub burnt_fee: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub amount: String,
    pub block_id: i64,
    pub datetime: String,
    #[serde(rename = "effective_gas_price")]
    pub effective_gas_price: String,
    #[serde(rename = "fee_payer")]
    pub fee_payer: String,
    pub from: String,
    #[serde(rename = "method_id")]
    pub method_id: String,
    pub status: TransactionStatus,
    pub to: String,
    #[serde(rename = "transaction_fee")]
    pub transaction_fee: String,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "transaction_index")]
    pub transaction_index: i64,
    #[serde(rename = "transaction_type")]
    pub transaction_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionStatus {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionsResponse {
    pub paging: Paging,
    pub results: Vec<Transaction>,
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    #[serde(rename = "current_page")]
    pub current_page: i64,
    pub last: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(rename = "total_page")]
    pub total_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct TransactionReceiptStatus {
    pub status: String,
    pub block_id: i64,
    pub tx_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionDetails {
    pub tx_hash: String,
    pub block_id: i64,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_used: String,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct ContractSourceCode {
    pub contract_address: String,
    pub source_code: String,
}

#[derive(Debug, Deserialize)]
pub struct LatestBlock {
    pub block_id: i64,
    pub datetime: String,
    pub hash: String,
    pub total_transaction_count: i64,
    pub block_reward: BlockReward,
}

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "contract_type")]
    pub contract_type: String,
    pub name: String,
    pub symbol: String,
    pub icon: String,
    pub decimal: i32,
    #[serde(rename = "total_supply")]
    pub total_supply: String,
    #[serde(rename = "total_transfers")]
    pub total_transfers: i64,
    #[serde(rename = "official_site")]
    pub official_site: String,
    #[serde(rename = "burn_amount")]
    pub burn_amount: String,
    #[serde(rename = "total_burns")]
    pub total_burns: i64,
}

pub struct KaiaScan {
    client: Client,
}

impl KaiaScan {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client })
    }

    async fn fetch_api<T>(&self, url: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let response = self
            .client
            .get(url)
            .header("Accept", "*/*")
            .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
            .send()
            .await
            .context("Failed to make HTTP request")?;

        let status = response.status();
        let body_text: serde_json::Value = response.json().await?;
        println!("📝 Response Status: {}", status);
        println!("📝 Response Body: {}", body_text);

        let api_response: T =
            serde_json::from_value(body_text).context("Failed to parse API response JSON")?;

        Ok(api_response)
    }

    pub async fn get_fungible_token(&self, token_address: Address) -> Result<TokenInfo> {
        let url = format!("{}{}/{}", BASE_URL, TOKENS_ENDPOINT, token_address.as_ref());
        self.fetch_api(&url).await
    }

    pub async fn get_nft_item(
        &self,
        nft_address: Address,
        token_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}{}?nftAddress={}&tokenId={}",
            BASE_URL,
            NFTS_ENDPOINT,
            nft_address.as_ref(),
            token_id
        );
        self.fetch_api(&url).await
    }

    pub async fn get_contract_creation_code(
        &self,
        contract_address: Address,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}api/v1/contracts/creation-code?contractAddress={}",
            BASE_URL,
            contract_address.as_ref()
        );
        self.fetch_api(&url).await
    }

    pub async fn get_latest_block(&self) -> Result<LatestBlock> {
        let url = format!("{}api/v1/blocks/latest", BASE_URL);
        self.fetch_api(&url).await
    }

    pub async fn get_block(&self, block_number: i64) -> Result<BlockDetails> {
        let url = format!("{}api/v1/blocks?blockNumber={}", BASE_URL, block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_blocks(&self) -> Result<BlocksResponse> {
        let url = format!("{}api/v1/blocks", BASE_URL);
        self.fetch_api(&url).await
    }

    pub async fn get_transactions_of_block(
        &self,
        block_number: i64,
    ) -> Result<TransactionsResponse> {
        let url = format!("{}api/v1/blocks/{}/transactions", BASE_URL, block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_transaction_receipt_status(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionReceiptStatus> {
        let url = format!(
            "{}api/v1/transaction-receipts/status?transactionHash={}",
            BASE_URL, transaction_hash
        );
        self.fetch_api(&url).await
    }

    pub async fn get_transaction(&self, transaction_hash: &str) -> Result<TransactionDetails> {
        let url = format!("{}api/v1/transactions/{}", BASE_URL, transaction_hash);
        self.fetch_api(&url).await
    }

    pub async fn get_contract_source_code(
        &self,
        contract_address: Address,
    ) -> Result<ContractSourceCode> {
        let url = format!(
            "{}api/v1/contracts/source-code?contractAddress={}",
            BASE_URL,
            contract_address.as_ref()
        );
        self.fetch_api(&url).await
    }

    pub async fn get_account_key_histories(
        &self,
        account_address: &str,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<AccountKeyHistoryResponse> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/accounts/{}/key-histories?page={}&size={}",
            BASE_URL, account_address, page, size
        );
        self.fetch_api(&url).await
    }

    pub async fn get_kaia_info(&self) -> Result<KaiaInfoResponse> {
        let url = format!("{}api/v1/kaia", BASE_URL);
        self.fetch_api(&url).await
    }

    pub async fn get_latest_block_burns(
        &self,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<BurnSummary> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/blocks/latest/burns?page={}&size={}",
            BASE_URL, page, size
        );
        self.fetch_api(&url).await
    }

    pub async fn get_latest_block_rewards(&self, block_number: i64) -> Result<BlockRewards> {
        let url = format!(
            "{}api/v1/blocks/latest/rewards?blockNumber={}",
            BASE_URL, block_number
        );
        self.fetch_api(&url).await
    }

    pub async fn get_block_burns(&self, block_number: i64) -> Result<BurnSummary> {
        let url = format!("{}api/v1/blocks/{}/burns", BASE_URL, block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_block_rewards(&self, block_number: i64) -> Result<BlockRewardsResponse> {
        let url = format!("{}api/v1/blocks/{}/rewards", BASE_URL, block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_internal_transactions_of_block(
        &self,
        block_number: i64,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<InternalTransactionsResponse> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/blocks/{}/internal-transactions?page={}&size={}",
            BASE_URL, block_number, page, size
        );
        self.fetch_api(&url).await
    }

    pub async fn get_transaction_status(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionStatus> {
        let url = format!(
            "{}api/v1/transactions/{}/status",
            BASE_URL, transaction_hash
        );
        self.fetch_api(&url).await
    }
}
