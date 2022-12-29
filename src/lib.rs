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
