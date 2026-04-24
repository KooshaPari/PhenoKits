---
layout: doc
title: Hello World Story
---

# Hello World: Your First Metron Operation

<StoryHeader
    title="First Operation"
    duration="2"
    difficulty="beginner"
    :gif="'/gifs/metron-hello-world.gif'"
/>

## Objective

Execute your first Metron operation successfully.

## Prerequisites

- Rust/Node/Python installed
- Metron CLI installed

## Implementation

```rust
use metron::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().await?;
    let result = client.hello().await?;
    println!("Success: {}", result);
    Ok(())
}
```

## Expected Output

```
Success: Hello from Metron!
```

## Next Steps

- [Core Integration](./core-integration)
- [API Reference](../reference/api)
