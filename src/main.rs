use std::error::Error;

use hypertx::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let credentials = hypertx::load_credentials();

    // Obtain a link_token by calling /link/token/create.
    match hypertx::get_link_token(&credentials).await {
        Ok(link) => println!("Worked! {:?}", link),
        Err(error) => println!("Doh!"),
    }

    println!("Credentials: {:?}", credentials);

    Ok(())
}
