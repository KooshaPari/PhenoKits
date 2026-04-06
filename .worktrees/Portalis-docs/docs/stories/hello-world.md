---
layout: doc
title: Hello World Story
---

# Hello World: Your First Portalis Operation

<StoryHeader
    title="First Operation"
    duration="2"
    difficulty="beginner"
    :gif="'/gifs/portalis-hello-world.gif'"
/>

## Objective

Execute your first Portalis operation successfully.

## Prerequisites

- Rust/Node/Python installed
- Portalis CLI installed

## Implementation

```rust
use portalis::Client;

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
Success: Hello from Portalis!
```

## Next Steps

- [Core Integration](./core-integration)
- [API Reference](../reference/api)
