use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = hypertx::load_configuration();

    // Obtain a link_token by calling /link/token/create.
    let link_token = hypertx::get_link_token(&config).await;

    println!("LinkToken: {:?}", link_token);

    Ok(())
}
