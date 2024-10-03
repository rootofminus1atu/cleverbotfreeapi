use std::io::Write;

use cleverbotfreeapi::CleverbotBuilder;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let c = CleverbotBuilder::default()
        .build().await?;

    loop {
        print!(">> ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let response = c.get_response(input).await?;
        println!("{}", response);
    }
}