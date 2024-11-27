use rustsdk::{Address, KaiaScan};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting KaiaScan SDK example...");

    let mainnet_client = KaiaScan::new(false)?;
    println!("✅ Mainnet Client initialized successfully");

    // Create client for testnet
    let testnet_client = KaiaScan::new(true)?;
    println!("✅ Testnet Client initialized successfully");

    let token_address = Address::new("0x5c74070fdea071359b86082bd9f9b3deaafbe32b");
    println!("🔍 Querying token address: {}", token_address.as_ref());

    // Get token info
    match mainnet_client.get_fungible_token(token_address).await {
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
    match mainnet_client.get_latest_block().await {
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

    match mainnet_client.get_transactions_of_block(block_number).await {
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

    match mainnet_client.get_kaia_info().await {
        Ok(info) => {
            println!("\n💎 Kaia Information:");
            println!("\nPrice Information:");
            println!("  USD Price: ${}", info.klay_price.usd_price);
            println!("  BTC Price: ₿{}", info.klay_price.btc_price);
            println!("  Market Cap: ${}", info.klay_price.market_cap);
            println!("  24h Price Change: {}%", info.klay_price.usd_price_changes);
            println!("  Volume: ${}", info.klay_price.volume);
            println!("  Total Supply: {}", info.klay_price.total_supply);

            println!("\nNetwork Summary:");
            println!("  1h Avg Block Time: {}", info.summary.avg_block_time1h);
            println!("  24h Avg Block Time: {}", info.summary.avg_block_time24h);
            println!(
                "  24h Avg Tx per Block: {:.2}",
                info.summary.avg_tx_per_block24h
            );
            println!("  Consensus Nodes: {}", info.summary.consensus_node);
        }
        Err(e) => {
            println!("❌ Error getting Kaia information: {}", e);
        }
    }

    // Get Block Rewards
    let block_number = 16973854;
    println!(
        "\n🏆 Querying block rewards for block number: {}",
        block_number
    );

    match mainnet_client.get_block_rewards(block_number).await {
        Ok(rewards) => {
            println!("\n📋 Block Rewards Information:");
            println!("Minted: {} KLAY", rewards.minted);
            println!("Total Fee: {} KLAY", rewards.total_fee);
            println!("Burnt Fee: {} KLAY", rewards.burnt_fee);

            println!("\nDistributions:");
            for dist in rewards.distributions {
                println!("  {} KLAY to {}", dist.amount, dist.distribution_type);
            }

            println!("\nRecipients:");
            for recipient in rewards.recipients {
                println!("\n  Name: {}", recipient.name);
                println!("  Address: {}", recipient.address);
                println!("  Amount: {} KLAY", recipient.amount);
                println!("  Type: {}", recipient.reward_type);
            }
        }
        Err(e) => {
            println!("❌ Error getting block rewards: {}", e);
        }
    }

    println!("\n🔥 Querying latest block burns...");
    match mainnet_client.get_block_burns(16973854).await {
        Ok(burns) => {
            println!("\n📋 Latest Block Burns Information:");
            println!("Accumulate Burnt: {}", burns.accumulate_burnt);
            println!("Accumulate Burnt Fees: {}", burns.accumulate_burnt_fees);
            println!("Accumulate Burnt Kaia: {}", burns.accumulate_burnt_kaia);
            println!("KIP103 Burnt: {}", burns.kip103_burnt);
            println!("KIP160 Burnt: {}", burns.kip160_burnt);
        }
        Err(e) => {
            println!("❌ Error getting latest block burns: {}", e);
        }
    }

    match mainnet_client.get_latest_block_burns(Some(2), Some(5)).await {
        Ok(burns_response) => {
            println!("\n📋 Latest Block Burns Information:");
            println!("Accumulated Burnt: {}", burns_response.accumulate_burnt);
            println!(
                "Accumulated Burnt Fees: {}",
                burns_response.accumulate_burnt_fees
            );
            println!(
                "Accumulated Burnt Kaia: {}",
                burns_response.accumulate_burnt_kaia
            );
            println!("KIP103 Burnt: {}", burns_response.kip103_burnt);
            println!("KIP160 Burnt: {}", burns_response.kip160_burnt);
        }

        Err(e) => {
            println!("❌ Error getting latest block burns: {}", e);
        }
    }

//mainnet_client.get_blocks(123456, None, None, None, None).await?;

match mainnet_client.get_blocks(block_number, None, None, Some(2), Some(5)).await {
    Ok(response) => {
        println!("\n📋 Blocks Information:");
      //  println!("Blocks: {:#?}", block);
        println!("Total Blocks: {}", response.results.len());
        
        for block in response.results {
        
        // for block in response.blocks {
            println!("\nBlock:");
            println!("  Block ID: {}", block.block_id);
            println!("  DateTime: {}", block.datetime);
            println!("  Total Transactions: {}", block.total_transaction_count);
            println!("  Block Reward:");
            println!("  Block Size: {}", block.block_size);
            println!("  Burnt Fees: {}", block.burnt_fees);
            println!("  Reward: {}", block.reward);
            println!("  Total Transactions: {}", block.total_transaction_count);
        }
    }
    Err(e) => {
        println!("❌ Error getting blocks: {}", e);
    }
}


    Ok(())
}
