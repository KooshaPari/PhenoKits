# Hello World Story

<StoryHeader
    title="Your First phenotype-docs-engine Operation"
    :duration="2"
    :gif="'/gifs/phenotype-docs-engine-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-docs-engine running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-docs-engine package installed

## Implementation

```rust
use phenotype-docs-engine::Client;

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
Success: Hello from phenotype-docs-engine!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
