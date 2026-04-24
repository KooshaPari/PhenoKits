# model

Data modeling and validation for TypeScript. Define schemas, enforce types.

## Features

- **Schemas**: Define data shapes
- **Validation**: Runtime type checking
- **Transformation**: Sanitize and transform
- **Serialization**: JSON, YAML, TOML

## Installation

```bash
npm install @model/core
```

## Usage

```typescript
import { model, string, number } from '@model/core';

const User = model({
  name: string().min(1).max(100),
  email: string().email(),
  age: number().int().min(0),
});

const user = User.parse({
  name: 'Alice',
  email: 'alice@example.com',
  age: 30,
});
```

## License

MIT
