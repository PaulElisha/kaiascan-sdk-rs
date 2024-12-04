use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const MAINNET_BASE_URL: &str = "https://mainnet-oapi.kaiascan.io/";
const TESTNET_BASE_URL: &str = "https://kairos-oapi.kaiscan.io/";

pub struct KaiaScan {
    client: Client,
    base_url: String,
}

const AUTH_TOKEN: &'static str = "";
const TOKENS_ENDPOINT: &str = "api/v1/tokens";
const NFTS_ENDPOINT: &str = "api/v1/nfts";

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

struct KeyHistoryEntry;
#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogEntry {
    pub results: Vec<EventLogResult>,

    pub pagings: Pagings,

    pub property: serde_json::Value, //  `HashMap<String, String>`
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogResult {
    pub log_index: u32,

    pub contract_address: String,

    pub log_type: String,

    pub topics: Vec<String>,

    pub data: String,

    pub items: Vec<EventItem>,

    pub block_number: u64,

    pub transaction_hash: String,

    pub estimated_event_log: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventItem {
    pub name: String,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagings {
    pub total_count: u32,

    pub current_page: u32,

    pub last: bool,

    pub total_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct TokenTransferEntry {
    #[serde(rename = "results")]
    pub results: Vec<TokenTransfer>,

    #[serde(rename = "paging")]
    pub paging: Paging,

    #[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenTransfer {
    pub contract: ContractInfo,

    #[serde(rename = "blockId")]
    pub block_id: u64,

    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,

    #[serde(rename = "feePayer")]
    pub fee_payer: String,

    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,

    pub datetime: String,

    pub from: String,

    pub to: String,

    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    pub contract_address: String,
    pub contract_type: String,
}

#[derive(Debug, Deserialize)]
pub struct NftBalanceEntry {
    pub results: Vec<NftBalance>,

    pub paging: Paging,

    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftBalance {
    pub contract: ContractInfo,

    #[serde(rename = "tokenId")]
    pub token_id: String,

    #[serde(rename = "tokenCount")]
    pub token_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct NftTransferEntry {
    #[serde(rename = "results")]
    pub results: Vec<NftTransfer>,

    #[serde(rename = "paging")]
    pub paging: Paging,

    #[serde(rename = "property")]
    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftTransfer {
    pub contract: ContractInfo,

    #[serde(rename = "blockId")]
    pub block_id: u64,

    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,

    #[serde(rename = "feePayer")]
    pub fee_payer: String,

    #[serde(rename = "transactionIndex")]
    pub transaction_index: u32,

    pub datetime: String,

    pub from: String,

    pub to: String,

    #[serde(rename = "tokenId")]
    pub token_id: String,

    #[serde(rename = "tokenCount")]
    pub token_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagin {
    #[serde(rename = "totalCount")]
    pub total_count: u32,

    #[serde(rename = "currentPage")]
    pub current_page: u32,

    pub last: bool,

    #[serde(rename = "totalPage")]
    pub total_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalanceEntry {
    pub results: Vec<TokenBalance>,

    pub paging: Page,

    pub property: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub contract: ContractInfo,

    pub balance: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename = "totalCount")]
    pub total_count: u32,

    #[serde(rename = "currentPage")]
    pub current_page: u32,

    pub last: bool,

    #[serde(rename = "totalPage")]
    pub total_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInputData {
    #[serde(rename = "originalValue")]
    pub original_value: String,

    #[serde(rename = "decodedValue")]
    pub decoded_value: Option<DecodedValue>,

    #[serde(rename = "utf8Value")]
    pub utf8_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedValue {
    pub signature: String,

    #[serde(rename = "methodId")]
    pub method_id: String,

    pub parameters: Vec<DecodedParameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedParameter {
    #[serde(rename = "type")]
    pub param_type: String,

    pub name: String,

    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransactionEntry {
    pub signature: String,

    pub method_id: String,

    pub parameters: Vec<TransactionParameter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionParameter {
    pub param_type: String,

    pub value: String,
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

#[derive(Debug, Deserialize)]
pub struct BlocksListResponse {
    pub paging: Paging,
    pub results: Vec<BlockListItem>,
}

#[derive(Debug, Deserialize)]
pub struct BlockListItem {
    #[serde(rename = "base_fee_per_gas")]
    pub base_fee_per_gas: String,
    #[serde(rename = "block_id")]
    pub block_id: i64,
    #[serde(rename = "block_proposer")]
    pub block_proposer: String,
    #[serde(rename = "block_size")]
    pub block_size: i64,
    #[serde(rename = "burnt_fees")]
    pub burnt_fees: String,
    pub datetime: String,
    pub reward: String,
    #[serde(rename = "total_transaction_count")]
    pub total_transaction_count: i64,
}

impl KaiaScan {
    pub fn new(is_testnet: bool) -> Result<Self> {
        let base_url = if is_testnet {
            TESTNET_BASE_URL.to_string()
        } else {
            MAINNET_BASE_URL.to_string()
        };

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, base_url })
    }

    async fn fetch_api<T>(&self, url: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let _url = format!("{}{}", self.base_url, url);

        let response = self
            .client
            .get(_url)
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
        let url = format!("{}/{}", TOKENS_ENDPOINT, token_address.as_ref());
        self.fetch_api(&url).await
    }

    pub async fn get_nft_item(
        &self,
        nft_address: Address,
        token_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}?nftAddress={}&tokenId={}",
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
        let endpoint = format!(
            "api/v1/contracts/creation-code?contractAddress={}",
            contract_address.as_ref()
        );
        self.fetch_api(&endpoint).await
    }
    pub async fn get_latest_block(&self) -> Result<LatestBlock> {
        let url = format!("api/v1/blocks/latest");
        self.fetch_api(&url).await
    }

    pub async fn get_block(&self, block_number: i64) -> Result<BlockDetails> {
        let url = format!("api/v1/blocks?blockNumber={}", block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_blocks(
        &self,
        block_number: i64,
        block_number_start: Option<i64>,
        block_number_end: Option<i64>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> Result<BlocksListResponse> {
        let page = page.unwrap_or(1).max(1);
        let size = size.unwrap_or(20).clamp(1, 2000);

        let mut query_params = vec![format!("blockNumber={}", block_number)];

        if let Some(start) = block_number_start {
            query_params.push(format!("blockNumberStart={}", start));
        }

        if let Some(end) = block_number_end {
            query_params.push(format!("blockNumberEnd={}", end));
        }

        query_params.push(format!("page={}", page));
        query_params.push(format!("size={}", size));

        let endpoint = format!("api/v1/blocks?{}", query_params.join("&"));
        self.fetch_api(&endpoint).await
    }

    pub async fn get_transactions_of_block(
        &self,
        block_number: i64,
    ) -> Result<TransactionsResponse> {
        let url = format!("api/v1/blocks/{}/transactions", block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_transaction_receipt_status(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionReceiptStatus> {
        let url = format!(
            "api/v1/transaction-receipts/status?transactionHash={}",
            transaction_hash
        );
        self.fetch_api(&url).await
    }

    pub async fn get_transaction(&self, transaction_hash: &str) -> Result<TransactionDetails> {
        let url = format!("api/v1/transactions/{}", transaction_hash);
        self.fetch_api(&url).await
    }

    pub async fn get_contract_source_code(
        &self,
        contract_address: Address,
    ) -> Result<ContractSourceCode> {
        let url = format!(
            "api/v1/contracts/source-code?contractAddress={}",
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
            "api/v1/accounts/{}/key-histories?page={}&size={}",
            account_address, page, size
        );
        self.fetch_api(&url).await
    }

    pub async fn get_kaia_info(&self) -> Result<KaiaInfoResponse> {
        let url = format!("api/v1/kaia");
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

        let url = format!("api/v1/blocks/latest/burns?page={}&size={}", page, size);
        self.fetch_api(&url).await
    }

    pub async fn get_latest_block_rewards(&self, block_number: i64) -> Result<BlockRewards> {
        let url = format!("api/v1/blocks/latest/rewards?blockNumber={}", block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_block_burns(&self, block_number: i64) -> Result<BurnSummary> {
        let url = format!("api/v1/blocks/{}/burns", block_number);
        self.fetch_api(&url).await
    }

    pub async fn get_block_rewards(&self, block_number: i64) -> Result<BlockRewardsResponse> {
        let url = format!("api/v1/blocks/{}/rewards", block_number);
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
            "api/v1/blocks/{}/internal-transactions?page={}&size={}",
            block_number, page, size
        );
        self.fetch_api(&url).await
    }

    pub async fn get_transaction_status(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionStatus> {
        let url = format!("api/v1/transactions/{}/status", transaction_hash);
        self.fetch_api(&url).await
    }

    // Method to get account event logs
    pub async fn get_account_event_logs(
        &self,
        account_address: Address,
        page: Option<u32>,
        size: Option<u32>,
        signature: Option<String>,
        block_number_start: Option<u64>,
        block_number_end: Option<u64>,
    ) -> Result<EventLogEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let mut query_params = vec![format!("page={}", page), format!("size={}", size)];

        if let Some(sig) = signature {
            query_params.push(format!("signature={}", sig));
        }

        if let Some(start) = block_number_start {
            query_params.push(format!("blockNumberStart={}", start));
        }

        if let Some(end) = block_number_end {
            query_params.push(format!("blockNumberEnd={}", end));
        }

        let query_string = query_params.join("&");
        let url = format!(
            "{}api/v1/accounts/{}/event-logs?{}",
            self.base_url,
            account_address.as_ref(),
            query_string
        );

        self.fetch_api(&url).await
    }

    // Method to get account NFT balances (KIP17)
    pub async fn get_account_kip17_nft_balances(
        &self,
        account_address: Address,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<NftBalanceEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/accounts/{}/nft-balances/kip17?page={}&size={}",
            self.base_url,
            account_address.as_ref(),
            page,
            size
        );

        self.fetch_api(&url).await
    }

    // Method to get account NFT balances (KIP37)
    pub async fn get_account_kip37_nft_balances(
        &self,
        account_address: Address,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<NftBalanceEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/accounts/{}/nft-balances/kip37?page={}&size={}",
            self.base_url,
            account_address.as_ref(),
            page,
            size
        );

        self.fetch_api(&url).await
    }

    pub async fn get_account_nft_transfers(
        &self,
        account_address: Address,
        page: Option<u32>,
        size: Option<u32>,
        contract_address: Option<Address>,
        block_number_start: Option<u64>,
        block_number_end: Option<u64>,
    ) -> Result<NftTransferEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let mut query_params = vec![format!("page={}", page), format!("size={}", size)];

        if let Some(contract) = contract_address {
            query_params.push(format!("contractAddress={}", contract.as_ref()));
        }

        if let Some(start) = block_number_start {
            query_params.push(format!("blockNumberStart={}", start));
        }

        if let Some(end) = block_number_end {
            query_params.push(format!("blockNumberEnd={}", end));
        }

        let query_string = query_params.join("&");
        let url = format!(
            "{}api/v1/accounts/{}/nft-transfers?{}",
            self.base_url,
            account_address.as_ref(),
            query_string
        );

        let raw_data: String = self.fetch_api(&url).await?;
        let nft_transfers: NftTransferEntry = serde_json::from_str(&raw_data)?;
        Ok(nft_transfers)
    }

    // Method to get account token balances
    pub async fn get_account_token_balances(
        &self,
        account_address: Address,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<TokenBalanceEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/accounts/{}/token-balances?page={}&size={}",
            self.base_url,
            account_address.as_ref(),
            page,
            size
        );

        self.fetch_api(&url).await
    }

    pub async fn get_transaction_input_data(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionInputData> {
        if transaction_hash.is_empty() {
            return Err(anyhow::anyhow!("Transaction hash is required"));
        }

        let url = format!(
            "{}api/v1/transactions/{}/input-data",
            self.base_url, transaction_hash
        );

        self.fetch_api(&url).await
    }

    pub async fn get_transaction_event_logs(
        &self,
        transaction_hash: &str,
        page: Option<u32>,
        size: Option<u32>,
        signature: Option<String>,
    ) -> Result<EventLogEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if transaction_hash.is_empty() {
            return Err(anyhow::anyhow!("Transaction hash is required"));
        }
        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let mut query_params = vec![format!("page={}", page), format!("size={}", size)];

        if let Some(sig) = signature {
            query_params.push(format!("signature={}", sig));
        }

        let query_string = query_params.join("&");
        let url = format!(
            "{}api/v1/transactions/{}/event-logs?{}",
            self.base_url, transaction_hash, query_string
        );

        self.fetch_api(&url).await
    }

    pub async fn get_transaction_internal_transactions(
        &self,
        transaction_hash: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<InternalTransactionEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if transaction_hash.is_empty() {
            return Err(anyhow::anyhow!("Transaction hash is required"));
        }
        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/transactions/{}/internal-transactions?page={}&size={}",
            self.base_url, transaction_hash, page, size
        );

        self.fetch_api(&url).await
    }

    pub async fn get_transaction_token_transfers(
        &self,
        transaction_hash: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<TokenTransferEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if transaction_hash.is_empty() {
            return Err(anyhow::anyhow!("Transaction hash is required"));
        }
        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/transactions/{}/token-transfers?page={}&size={}",
            self.base_url, transaction_hash, page, size
        );

        self.fetch_api(&url).await
    }

    pub async fn get_transaction_nft_transfers(
        &self,
        transaction_hash: &str,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Result<NftTransferEntry> {
        let page = page.unwrap_or(1);
        let size = size.unwrap_or(20);

        if transaction_hash.is_empty() {
            return Err(anyhow::anyhow!("Transaction hash is required"));
        }
        if page < 1 {
            return Err(anyhow::anyhow!("Page must be >= 1"));
        }
        if size < 1 || size > 2000 {
            return Err(anyhow::anyhow!("Size must be between 1 and 2000"));
        }

        let url = format!(
            "{}api/v1/transactions/{}/nft-transfers?page={}&size={}",
            self.base_url, transaction_hash, page, size
        );

        self.fetch_api(&url).await
    }
}
