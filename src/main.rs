use std::error::Error;

use hypertx::Credentials;
use hypertx::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = hypertx::load_configuration();

    // Obtain a link_token by calling /link/token/create.
    match hypertx::get_link_token(&config.credentials).await {
        Ok(link) => println!("Worked! {:?}", link),
        Err(error) => println!("Doh!"),
    }

    println!("Config: {:?}", config);

    // Get public_token, exchange for access_token.
    
     
    Ok(())
}
