use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = get_credentials();

    // Obtain a link_token by calling /link/token/create.
    match get_link_token(&credentials).await {
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
    let transactions = nudge_transaction(transactions);

    println!("Credentials: {:?}", credentials);
    println!("Transactions: {:?}", transactions);

    Ok(())
}

fn read_credentials_from_env() -> Result<Credentials, env::VarError> {
    dotenv().ok();

    Ok(Credentials {
        client_id: env::var("CLIENT_ID")?,
        client_secret: env::var("CLIENT_SECRET")?,
        environment: String::from("prod")
    })
}

pub fn get_credentials() -> Credentials {
    let credentials = read_credentials_from_env();

    match credentials {
        Ok(credentials) => credentials,
        Err(var_error) => panic!("Missing credentials in the `.env` file.")
    }
}

async fn get_link_token(credentials: &Credentials) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    Ok(resp)
}

pub fn nudge_transaction(transactions: [Transaction; 2]) -> [Transaction; 2] {
    println!("Nudged!");

    // Inspect transactions near the beginning or end of a month and nudge.
    // Return the full transaction set.
    transactions
}

// Types
#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub environment: String
}

#[derive(Debug)]
pub struct Transaction {
    transaction_date: String,
    amount: f32,
    category: Category,
    vendor: String
}

#[derive(Debug)]
pub enum Category {
    Groceries,
    EatingOut,
    Rent,
    ChildCare
}
