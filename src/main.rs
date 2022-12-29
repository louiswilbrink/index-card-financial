use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = hypertx::load_configuration();

    Ok(())
}
