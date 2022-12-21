use dotenv::dotenv;
use std::env;

use std::error::Error;
use std::collections::HashMap;

use env::var;
use env::VarError;

// Types
#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String
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
        Err(var_error) => panic!("Missing environment variables in the `.env` file.")
    }
}

fn read_environment_variables() -> Result<Configuration, VarError> {
    dotenv().ok();

    Ok(Configuration {
        credentials: Credentials {
            client_id: var("CLIENT_ID")?,
            client_secret: var("CLIENT_SECRET")?
        },
        environment: var("ENV")?,
        environment_url: var("ENV_URL")?
    })
}

pub async fn get_link_token(credentials: &Credentials) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    Ok(resp)
}
