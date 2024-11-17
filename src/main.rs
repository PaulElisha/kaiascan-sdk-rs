use rustsdk::{Address, KaiaScan};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting KaiaScan SDK example...");

    let client = KaiaScan::new()?;
    println!("✅ Client initialized successfully");

    let token_address = Address::new("0x5c74070fdea071359b86082bd9f9b3deaafbe32b");
    println!("🔍 Querying token address: {}", token_address.as_ref());

    // Get token info
    match client.get_fungible_token(token_address).await {
        Ok(response) => {
            println!("\n📋 Token Information:");
            println!("Name: {}", response.name);
            println!("Symbol: {}", response.symbol);
            println!("Total Supply: {}", response.total_supply);
            println!("Contract Type: {}", response.contract_type);
            println!("Official Site: {}", response.official_site);
        }
        Err(e) => {
            println!("❌ Error getting token information: {}", e);
        }
    }

    // latest block
    match client.get_latest_block().await {
        Ok(block) => {
            println!("\n🔲 Latest Block Information:");
            println!("Block ID: {}", block.block_id);
            println!("DateTime: {}", block.datetime);
            println!("Hash: {}", block.hash);
            println!("Total Transactions: {}", block.total_transaction_count);
            println!("Block Reward:");
            println!("  Minted: {}", block.block_reward.minted);
            println!("  Total Fee: {}", block.block_reward.total_fee);
            println!("  Burnt Fee: {}", block.block_reward.burnt_fee);
        }
        Err(e) => {
            println!("❌ Error getting latest block: {}", e);
        }
    }


    let block_number = 16973854;
    println!(
        "\n🔍 Querying transactions of block number: {}",
        block_number
    );

    match client.get_transactions_of_block(block_number).await {
        Ok(response) => {
            println!("\n📋 Transactions Information:");
            println!("Paging: {:#?}", response.paging);

            for transaction in response.results {
                println!("\nTransaction:");
                println!("  Hash: {}", transaction.transaction_hash);
                println!("  From: {}", transaction.from);
                println!("  To: {}", transaction.to);
                println!("  Amount: {}", transaction.amount);
                println!("  Block ID: {}", transaction.block_id);
                println!("  DateTime: {}", transaction.datetime);
                println!("  Effective Gas Price: {}", transaction.effective_gas_price);
                println!("  Fee Payer: {}", transaction.fee_payer);
                println!("  Method ID: {}", transaction.method_id);
                println!("  Status: {}", transaction.status.status);
                println!("  Transaction Fee: {}", transaction.transaction_fee);
                println!("  Transaction Index: {}", transaction.transaction_index);
                println!("  Transaction Type: {}", transaction.transaction_type);
            }
        }
        Err(e) => {
            println!("❌ Error getting transactions: {}", e);
        }
    }

    Ok(())
}
