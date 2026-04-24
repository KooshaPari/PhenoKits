# relay

Cursor-based pagination for any data source. Works with SQL, NoSQL, APIs.

## Features

- **Cursor-based**: Efficient pagination without offset
- **Any Source**: SQL, MongoDB, REST APIs
- **Total Count**: Optional total count support
- **Bidirectional**: Forward and backward cursors

## Installation

```bash
npm install @relay/pagination
```

## Usage

```typescript
import { createPaginatedQuery } from '@relay/pagination';

const query = createPaginatedQuery({
  select: '*',
  from: 'users',
  cursor: { id: 'desc' },
  limit: 20,
});

// First page
const page1 = await query.execute();
// Next page
const page2 = await query.next();
// Previous
const page1Again = await query.prev();
```

## Architecture

```
src/
├── cursor/       # Cursor encoding/decoding
├── adapters/    # SQL, MongoDB, REST
└── types/      # TypeScript types
```

## License

MIT
