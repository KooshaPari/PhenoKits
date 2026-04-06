# Hello World Story

<StoryHeader
    title="Your First phenotype-nexus Operation"
    :duration="2"
    :gif="'/gifs/phenotype-nexus-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-nexus running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-nexus package installed

## Implementation

```rust
use phenotype-nexus::Client;

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
Success: Hello from phenotype-nexus!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
