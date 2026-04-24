# Quick Start Journey

<UserJourney
    :steps="[
        { title: 'Install', desc: 'Install phenotype-logging-zig package' },
        { title: 'Configure', desc: 'Set up basic configuration' },
        { title: 'Run', desc: 'Execute first operation' }
    ]"
    :duration="5"
    :gif-src="'/gifs/phenotype-logging-zig-quickstart.gif'"
/>

## Step-by-Step

### 1. Installation

```bash
# Install via cargo/npm/pip
cargo add phenotype-logging-zig
# or
npm install @phenotype-logging-zig/core
```

### 2. Basic Configuration

```yaml
# config.yaml
name: my-project
environment: development
```

### 3. First Operation

```rust
// Example usage
use phenotype-logging-zig::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await;
    let result = client.process().await;
    println!("{:?}", result);
}
```

## Verification

Run the built-in health check:

```bash
cargo test --lib
```
