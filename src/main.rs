use solana_sdk::transaction::Transaction;

pub mod settle;
use settle::run;

pub fn main() {
    // implement an actual transaction
    let transaction = Transaction::default();

    // sends tx to chain
    run(transaction).unwrap();

}
