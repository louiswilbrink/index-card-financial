use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = hypertx::load_configuration();

    let transactions = hypertx::get_transactions(&config).await;

    println!("transactions: {:?}", transactions);

    Ok(())
}
