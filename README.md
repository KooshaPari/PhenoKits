# pheno-core-cgo

Go CGO bindings for Rust `phenotype-core`.

## Prerequisites

- Rust toolchain
- Go 1.22+
- cbindgen (for C header generation)

## Building

```bash
# Generate C headers from Rust
cd PhenoKit/rust/phenotype-core
cbindgen --lang c --crate phenotype-core --output ../../go/pheno-core-cgo/phenotype_core.h

# Build the Rust library as cdylib
cargo build --release

# Build Go package
cd PhenoKit/go/pheno-core-cgo
go build
```

## Usage

```go
package main

import (
    "fmt"
    "log"
    
    phenocore "github.com/KooshaPari/PhenoKit/go/pheno-core-cgo"
)

func main() {
    entity := phenocore.NewEntityId("123", "user")
    
    if entity.Validate() {
        json, err := entity.ToJSON()
        if err != nil {
            log.Fatal(err)
        }
        fmt.Println(json) // {"id":"123","namespace":"user"}
    }
}
```
