use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

use env::var;
use env::VarError;

use serde::{Serialize, Deserialize};

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
pub struct LinkToken {
    expiration: String,
    link_token: String,
    request_id: String
}

#[derive(Debug)]
pub enum Category {
    Groceries,
    EatingOut,
    Rent,
    ChildCare
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkTokenUser {
    client_user_id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkTokenRequestBody<'a> {
    client_id: &'a str,
    secret: &'a str,
    client_name: String,
    country_codes: [String; 1],
    language: String,
    user: LinkTokenUser,
    products: [String; 1]
}

impl LinkTokenRequestBody<'_> {
    fn new<'a>(client_id: &'a str, client_secret: &'a str) -> LinkTokenRequestBody<'a> {
        let user = LinkTokenUser {
            client_user_id: String::from("unique_user_id")
        };

        LinkTokenRequestBody {
            client_id: client_id,
            secret: client_secret,
            client_name: String::from("Insert Client name here"),
            country_codes: [String::from("US")],
            language: String::from("en"),
            user,
            products: [String::from("auth")]
        }
    }
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
            client_secret: var("CLIENT_SECRET")?
        },
        environment: var("ENV")?,
        environment_url: var("ENV_URL")?
    })
}

pub async fn get_link_token(config: &Configuration) -> LinkToken {
    let link_token = match request_link_token(&config).await {
        Ok(link_token) => link_token,
        Err(error) => panic!("Error requesting link token: {:?}", error)
    };

    link_token
}

async fn request_link_token(config: &Configuration) -> Result<LinkToken, reqwest::Error> {
    let client_id = &config.credentials.client_id;
    let client_secret = &config.credentials.client_secret;

    let url = format!("{}/{}", config.environment_url, "link/token/create");

    let client = reqwest::Client::new();

    let link_token_request_body = LinkTokenRequestBody::new(&client_id, &client_secret);

    let response = client.post(url)
        .json(&link_token_request_body)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let link_token = LinkToken {
        expiration: response.get("expiration").unwrap().to_string(),
        link_token: response.get("link_token").unwrap().to_string(),
        request_id: response.get("request_id").unwrap().to_string()
    };
    
    Ok(link_token)

}
