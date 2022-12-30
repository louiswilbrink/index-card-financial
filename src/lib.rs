use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

use env::var;
use env::VarError;

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String
}

#[derive(Debug)]
pub struct Configuration {
    pub credentials: Credentials,
    pub environment: String,
    pub environment_url: String
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

#[derive(Serialize, Deserialize, Debug)]
struct TransactionRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    start_date: String,
    end_date: String
}

pub fn load_configuration() -> Configuration {
    let config = read_environment_variables();

    match config {
        Ok(config) => config,
        Err(_) => panic!("Missing environment variables in the `.env` file.")
    }
}

fn read_environment_variables() -> Result<Configuration, VarError> {
    dotenv().ok();

    Ok(Configuration {
        credentials: Credentials {
            client_id: var("CLIENT_ID")?,
            client_secret: var("CLIENT_SECRET")?,
            access_token: var("ACCESS_TOKEN")?
        },
        environment: var("ENV")?,
        environment_url: var("ENV_URL")?
    })
}

pub async fn get_transactions(config: &Configuration) -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();

    retrieve_transactions(&config).await;

    transactions
}

pub async fn retrieve_transactions(config: &Configuration) -> Result<Vec<Transaction>, reqwest::Error> {
    let mut transactions: Vec<Transaction> = Vec::new();

    let url = format!("{}/{}", config.environment_url, "transactions/get");

    println!("url: {}", url);

    let transaction_request_body = TransactionRequest {
        client_id: &config.credentials.client_id,
        secret: &config.credentials.client_secret,
        access_token: &config.credentials.access_token,
        start_date: String::from("2021-01-01"),
        end_date: String::from("2021-12-10")
    };

    let client = reqwest::Client::new();

    let response = client.post(url)
        .json(&transaction_request_body)
        .send()
        .await
        .unwrap();

    println!("Response: {:?}", response);

    Ok(transactions)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
