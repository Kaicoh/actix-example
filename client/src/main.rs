use models::Hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("http://localhost:8000")
        .await?
        .json::<Hello>()
        .await?;

    println!("body = {:?}", body);
    Ok(())
}
