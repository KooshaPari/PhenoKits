# Hello World Story

<StoryHeader
    title="Your First phenotype-cli-extensions Operation"
    :duration="2"
    :gif="'/gifs/phenotype-cli-extensions-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-cli-extensions running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-cli-extensions package installed

## Implementation

```rust
use phenotype-cli-extensions::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let client = Client::new().await?;
    
    // Execute operation
    let result = client.hello().await?;
    
    println!("Success: {}", result);
    Ok(())
}
```

## Expected Output

```
Success: Hello from phenotype-cli-extensions!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
