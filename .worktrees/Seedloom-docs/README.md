# seed

Database seeding framework for TypeScript. Fixtures, factories, and generators.

## Features

- **Factories**: Create test data
- **Fixtures**: Reusable data sets
- **Generators**: Faker integration
- **Transactions**: Safe seeding

## Installation

```bash
npm install @seed/database
```

## Usage

```typescript
import { factory, fixture } from '@seed/database';

const userFactory = factory('User', {
  name: faker.name,
  email: faker.email,
});

const admin = await userFactory.create({ role: 'admin' });
const users = await userFactory.createMany(10);
```

## License

MIT
