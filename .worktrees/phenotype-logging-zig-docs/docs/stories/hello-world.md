# Hello World Story

<StoryHeader
    title="Your First phenotype-logging-zig Operation"
    :duration="2"
    :gif="'/gifs/phenotype-logging-zig-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-logging-zig running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-logging-zig package installed

## Implementation

```rust
use phenotype-logging-zig::Client;

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
Success: Hello from phenotype-logging-zig!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
