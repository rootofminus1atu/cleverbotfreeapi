use cleverbotfreeapi::CleverbotBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let c = CleverbotBuilder::default()
        .with_custom_history_size(5)
        .build().await?;

    let r = c.get_response("are you a bot").await?;
    println!("response: {r}");

    Ok(())
}