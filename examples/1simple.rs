use cleverbotfreeapi::CleverbotBuilder;


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let c = CleverbotBuilder::default()
        .with_custom_history_size(5)
        .build().await?;

    let r = c.get_response("are you a bot").await?;
    println!("response: {r}");
    let r = c.get_response("who are you then").await?;
    println!("response: {r}");
    let r = c.get_response("ok").await?;
    println!("response: {r}");
    println!("c: {:?}", (*c.history.read()).clone().into_iter().collect::<Vec<_>>());



    Ok(())
}