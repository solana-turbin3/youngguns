use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;


// Settle the state on solana, called by sequencer
pub fn settle_state(proof: String) -> Result<String> {
    let rpc_client = RpcClient::new::<&str>("http://localhost:8899".into());

    // TODO: Create transaction, still to be implemented
    let transaction = Transaction::default();

    println!("proof: {}", proof);

    // Send transaction on chain - Currently will error with Invalid transaction. 
    let settle_tx_hash = rpc_client
        .send_and_confirm_transaction(&transaction)
        ?;
    Ok(settle_tx_hash.to_string())
}

pub fn run(transaction: Transaction) -> Result<()> {
    
    let mut tx_counter = 0u32;

    // let accounts_to_lock = transaction.message.account_keys.clone();

    // kept counter just to do something, replace with actual logic
    tx_counter += 1;
    println!("tx_counter: {}", tx_counter);

    let _settle_tx_hash = settle_state("proof".into());

    println!("settle_tx_hash: {:?}", _settle_tx_hash);
    Ok(())
}