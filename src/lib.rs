use dotenv::dotenv;
use std::env;

use std::error::Error;
use std::collections::HashMap;

// Types
#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub environment: String
}

#[derive(Debug)]
pub struct Transaction {
    pub transaction_date: String,
    pub amount: f32,
    pub category: Category,
    pub vendor: String
}

#[derive(Debug)]
pub enum Category {
    Groceries,
    EatingOut,
    Rent,
    ChildCare
}

pub fn load_credentials() -> Credentials {
    let credentials = read_credentials_from_env();

    match credentials {
        Ok(credentials) => credentials,
        Err(var_error) => panic!("Missing credentials in the `.env` file.")
    }
}

fn read_credentials_from_env() -> Result<Credentials, env::VarError> {
    dotenv().ok();

    Ok(Credentials {
        client_id: env::var("CLIENT_ID")?,
        client_secret: env::var("CLIENT_SECRET")?,
        environment: String::from("prod")
    })
}

pub async fn get_link_token(credentials: &Credentials) -> Result<HashMap<String, String>, Box<dyn Error>> {
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

