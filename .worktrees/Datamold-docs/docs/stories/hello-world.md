---
layout: doc
title: Hello World Story
---

# Hello World: Your First Datamold Operation

<StoryHeader
    title="First Operation"
    duration="2"
    difficulty="beginner"
    :gif="'/gifs/datamold-hello-world.gif'"
/>

## Objective

Execute your first Datamold operation successfully.

## Prerequisites

- Rust/Node/Python installed
- Datamold CLI installed

## Implementation

```rust
use datamold::Client;

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
Success: Hello from Datamold!
```

## Next Steps

- [Core Integration](./core-integration)
- [API Reference](../reference/api)
