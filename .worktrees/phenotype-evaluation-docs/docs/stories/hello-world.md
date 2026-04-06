# Hello World Story

<StoryHeader
    title="Your First phenotype-evaluation Operation"
    :duration="2"
    :gif="'/gifs/phenotype-evaluation-hello-world.gif'"
    difficulty="beginner"
/>

## Objective

Get phenotype-evaluation running with a basic operation.

## Prerequisites

- Rust/Node/Python installed
- phenotype-evaluation package installed

## Implementation

```rust
use phenotype-evaluation::Client;

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
Success: Hello from phenotype-evaluation!
```

## Next Steps

- [Core Integration](./core-integration)
- Read [API Reference](../reference/api)
