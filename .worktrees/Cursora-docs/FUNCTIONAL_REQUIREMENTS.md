# Functional Requirements: relay

## FR-RLY-001: Connection Type
FR-RLY-001a: The library SHALL export `Connection<T>` with: `edges: Edge<T>[]`, `pageInfo: PageInfo`, `totalCount?: number`.
FR-RLY-001b: `Edge<T>` SHALL have: `node: T`, `cursor: string`.
FR-RLY-001c: `PageInfo` SHALL have: `hasNextPage: boolean`, `hasPreviousPage: boolean`, `startCursor: string | null`, `endCursor: string | null`.

## FR-RLY-002: Pagination Arguments
FR-RLY-002a: Forward pagination SHALL use `{ first: number, after?: string }`.
FR-RLY-002b: Backward pagination SHALL use `{ last: number, before?: string }`.
FR-RLY-002c: Using both `first`/`after` and `last`/`before` in the same call SHALL throw `InvalidPaginationArguments`.
FR-RLY-002d: `first` and `last` SHALL be clamped to a configurable maximum (default: 100).

## FR-RLY-003: Cursor Encoding
FR-RLY-003a: Cursors SHALL be opaque base64-encoded strings encoding the sort key value(s).
FR-RLY-003b: Clients SHALL NOT parse or construct cursor strings.
FR-RLY-003c: Invalid cursor format SHALL result in `InvalidCursorError` with a descriptive message.

## FR-RLY-004: SQL Adapter
FR-RLY-004a: `createSQLPaginator({ query, cursor, limit })` SHALL return a function `paginate(args) -> Promise<Connection<T>>`.
FR-RLY-004b: The SQL adapter SHALL inject a `WHERE cursor_column > ?` clause for forward pagination.
FR-RLY-004c: The SQL adapter SHALL support multi-column cursors for stable ordering (e.g., `(created_at, id)`).
FR-RLY-004d: The Prisma adapter SHALL accept a Prisma model delegate and `orderBy` configuration.

## FR-RLY-005: Total Count
FR-RLY-005a: Total count SHALL be optional. It is computed only when `includeTotalCount: true` is passed.
FR-RLY-005b: The SQL adapter SHALL execute a `SELECT COUNT(*)` query in parallel with the data query when total count is requested.
FR-RLY-005c: MongoDB adapter total count SHALL use `estimatedDocumentCount()` for performance.

## FR-RLY-006: Bidirectional Pagination
FR-RLY-006a: `last` + `before` pagination SHALL return items before the cursor in reverse order, re-sorted to ascending for the response.
FR-RLY-006b: `hasPreviousPage` SHALL be true when using forward pagination with items before the first item.
FR-RLY-006c: `hasNextPage` SHALL be true when using backward pagination with items after the last item.

## FR-RLY-007: Type Safety
FR-RLY-007a: The library SHALL be fully typed. `createSQLPaginator<UserRow>()` SHALL return `Connection<UserRow>`.
FR-RLY-007b: Cursor fields SHALL be type-checked against the entity's column types at definition time where possible.
