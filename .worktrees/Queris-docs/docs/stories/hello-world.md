---
layout: doc
title: Hello World Story
---

# Hello World: Your First Queris Operation

<StoryHeader
    title="First Operation"
    duration="2"
    difficulty="beginner"
    :gif="'/gifs/queris-hello-world.gif'"
/>

## Objective

Execute your first Queris operation successfully.

## Prerequisites

- Rust/Node/Python installed
- Queris CLI installed

## Implementation

```rust
use queris::Client;

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
Success: Hello from Queris!
```

## Next Steps

- [Core Integration](./core-integration)
- [API Reference](../reference/api)
