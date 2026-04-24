# Hello World Story

<StoryHeader
    title="Your First phenotype-types Operation"
    :duration="2"
    :gif="'/gifs/phenotype-types-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-types running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-types package installed

## Implementation

```rust
use phenotype-types::Client;

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
Success: Hello from phenotype-types!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
