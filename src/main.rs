use std::error::Error;

use hypertx::Credentials;
use hypertx::Transaction;
use hypertx::Category;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = hypertx::load_credentials();

    // Obtain a link_token by calling /link/token/create.
    match hypertx::get_link_token(&credentials).await {
        Ok(link) => println!("Worked! {:?}", link),
        Err(error) => println!("Doh!"),
    }

    // Get transactions from all accounts.

    let transactions: [Transaction; 2] = [{ Transaction {
        transaction_date: String::from("December 7, 2022"),
        amount: 32.51,
        category: Category::Groceries,
        vendor: String::from("Trader Joe's")
    } }, { Transaction {
        transaction_date: String::from("December 1, 2022"),
        amount: 154.02,
        category: Category:: Groceries,
        vendor: String::from("Whole Foods")
    } }];

    // Nudge transactions close to the beginning/end of the month.
    let transactions = hypertx::nudge_transaction(transactions);

    println!("Credentials: {:?}", credentials);
    println!("Transactions: {:?}", transactions);

    Ok(())
}

