# cleverbotfreeapi

`cleverbotfreeapi` is a Rust client library for interacting with a washed down version of the Cleverbot API for free.
This crate provides easy-to-use interfaces for sending messages to Cleverbot and receiving responses. Suitable for async builds like web servers or bots.

## Features

- [x] Get cleverbot responses.
- [x] Maintain conversation history.
- [x] Retry failed requests automatically (configurable).

## Usage

Here is a basic example of using `cleverbotfreeapi` to send a message to Cleverbot and receive a response:

```rust
use cleverbotfreeapi::{CleverbotBuilder, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cleverbot = CleverbotBuilder::default().build().await?;
    let response = cleverbot.get_response("are you a bot").await?;
    println!("response: {}", response);  // would respond with something like "no, I'm human"
    Ok(())
}
```

For more detailed examples, see the [examples](https://github.com/rootofminus1atu/cleverbotfreeapi/tree/main/examples) directory in the repository.

## Contributing

Contributions are welcome!

## Potential future features

- Sessions, to automatically manage different conversations happening in different "places". Not sure if this is a good idea, as some people might be happy with im-memory sessions, some might want to use a db, etc.
- Tests

## Links

- [Documentation](https://docs.rs/cleverbotfreeapi)
- [Crates.io](https://crates.io/crates/cleverbotfreeapi)
- [Repository](https://github.com/rootofminus1atu/cleverbotfreeapi)

