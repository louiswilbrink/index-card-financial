use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = get_credentials();

    println!("{:?}", credentials);

    // Obtain a link_token by calling /link/token/create.

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
        Err(VarError) => panic!("Missing credentials in the `.env` file.")
    }
}

// Types
#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub environment: String
}
