# Phenotype Types

> Type System Foundation for Phenotype Ecosystem

A comprehensive TypeScript type system providing shared interfaces, domain models, and type definitions across all Phenotype services.

## Philosophy

**Type safety should be pervasive, not optional.**

- **Shared**: Single source of truth for all type definitions
- **Strict**: Maximum type safety with strict TypeScript
- **Documented**: Types should be self-documenting
- **Composed**: Complex types built from simple primitives
- **Validated**: Runtime validation matches static types

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| **Core Types** | Agent, Task, Skill, Workflow definitions | Stable |
| **Domain Models** | Event, Message, State, Context | Stable |
| **Validation** | Zod schemas for runtime validation | Stable |
| **Serialization** | JSON/Protobuf compatible types | Stable |
| **Generics** | Higher-kinded type utilities | Stable |
| **Branding** | Nominal typing for type safety | Beta |
| **Versioning** | Type evolution and migration | Beta |

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Phenotype Types System                                 │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         Core Domain Types                              │   │
│  │                                                                      │   │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │   │
│  │   │  Agent   │  │  Task    │  │  Skill   │  │ Workflow │           │   │
│  │   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘           │   │
│  │        │             │             │             │                    │   │
│  │        └─────────────┴─────────────┴─────────────┘                    │   │
│  │                      │                                                  │   │
│  └──────────────────────┼───────────────────────────────────────────────────┘   │
│                         │                                                   │
│  ┌──────────────────────┼───────────────────────────────────────────────────┐ │
│  │                    Infrastructure Types                                  │ │
│  │                                                                      │ │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │ │
│  │   │  Event   │  │  Message │  │  State   │  │ Context  │           │ │
│  │   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘           │ │
│  │        │             │             │             │                    │ │
│  │        └─────────────┴─────────────┴─────────────┘                    │ │
│  │                      │                                                  │ │
│  └──────────────────────┼───────────────────────────────────────────────────┘ │
│                         │                                                   │
│  ┌──────────────────────┼───────────────────────────────────────────────────┐ │
│  │                    Validation & Utilities                              │ │
│  │                                                                      │ │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐           │ │
│  │   │  Zod     │  │  Guards  │  │  Branded │  │ Result   │           │ │
│  │   │  Schemas │  │  Type    │  │  Types   │  │ Type     │           │ │
│  │   └──────────┘  └──────────┘  └──────────┘  └──────────┘           │ │
│  │                                                                      │ │
│  └──────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Core Types

### Agent Types

```typescript
interface Agent {
  id: AgentId;
  type: AgentType;
  provider: LLMProvider;
  model: string;
  config: AgentConfig;
  capabilities: Capability[];
  state: AgentState;
}

type AgentId = Branded<string, 'AgentId'>;
type AgentType = 'coding' | 'research' | 'execution' | 'general';

interface AgentConfig {
  temperature: number;
  maxTokens: number;
  timeout: Duration;
  retryPolicy: RetryPolicy;
  tools: ToolName[];
}

enum AgentState {
  Idle = 'idle',
  Running = 'running',
  Paused = 'paused',
  Error = 'error',
  Terminated = 'terminated'
}
```

### Task Types

```typescript
interface Task {
  id: TaskId;
  name: string;
  description?: string;
  type: TaskType;
  status: TaskStatus;
  priority: Priority;
  dependencies: TaskId[];
  input: JsonValue;
  output?: JsonValue;
  error?: TaskError;
  timing: TaskTiming;
  metadata: TaskMetadata;
}

type TaskId = Branded<string, 'TaskId'>;
type TaskType = 'simple' | 'composite' | 'parallel' | 'conditional' | 'loop';

type TaskStatus = 
  | 'pending'
  | 'scheduled'
  | 'running'
  | 'completed'
  | 'failed'
  | 'cancelled'
  | 'retrying';

interface TaskTiming {
  createdAt: Timestamp;
  scheduledAt?: Timestamp;
  startedAt?: Timestamp;
  completedAt?: Timestamp;
  duration?: Duration;
}
```

### Skill Types

```typescript
interface Skill {
  id: SkillId;
  name: string;
  version: SemVer;
  type: SkillType;
  signature: SkillSignature;
  implementation: SkillImplementation;
  metadata: SkillMetadata;
}

type SkillId = Branded<string, 'SkillId'>;
type SkillType = 'native' | 'python' | 'wasm' | 'typescript';

interface SkillSignature {
  input: JsonSchema;
  output: JsonSchema;
  errors: ErrorType[];
}

interface SkillImplementation {
  type: SkillType;
  entrypoint: string;
  dependencies: Dependency[];
  resources: ResourceRequirements;
}
```

### Workflow Types

```typescript
interface Workflow {
  id: WorkflowId;
  name: string;
  version: SemVer;
  tasks: Map<TaskId, Task>;
  dependencies: DAG<TaskId>;
  triggers: Trigger[];
  config: WorkflowConfig;
}

type WorkflowId = Branded<string, 'WorkflowId'>;

interface WorkflowConfig {
  maxConcurrency: number;
  timeout: Duration;
  retryPolicy: RetryPolicy;
  errorHandling: ErrorHandling;
}

type ErrorHandling = 
  | { type: 'stop' }
  | { type: 'continue'; on: TaskStatus[] }
  | { type: 'compensate'; compensations: Map<TaskId, TaskId> };
```

## Infrastructure Types

### Event Types

```typescript
interface Event {
  id: EventId;
  type: EventType;
  source: ServiceId;
  timestamp: Timestamp;
  data: JsonValue;
  metadata: EventMetadata;
}

type EventId = Branded<string, 'EventId'>;
type EventType = 
  | 'agent.created'
  | 'agent.started'
  | 'agent.completed'
  | 'agent.failed'
  | 'task.created'
  | 'task.completed'
  | 'task.failed'
  | 'workflow.started'
  | 'workflow.completed';

interface EventMetadata {
  correlationId: CorrelationId;
  causationId?: EventId;
  traceId: TraceId;
  spanId: SpanId;
}
```

### Message Types

```typescript
interface Message<T = unknown> {
  id: MessageId;
  type: MessageType;
  payload: T;
  headers: MessageHeaders;
  timestamp: Timestamp;
}

type MessageId = Branded<string, 'MessageId'>;
type MessageType = 
  | 'command'
  | 'query'
  | 'event'
  | 'notification';

interface MessageHeaders {
  version: number;
  priority: Priority;
  ttl?: Duration;
  traceContext: TraceContext;
}
```

### State Types

```typescript
interface State<T> {
  value: T;
  version: number;
  timestamp: Timestamp;
  metadata: StateMetadata;
}

interface StateMetadata {
  owner: ServiceId;
  checksum: string;
  previousVersion?: number;
}

type StateChange<T> = 
  | { type: 'created'; state: State<T> }
  | { type: 'updated'; previous: State<T>; current: State<T> }
  | { type: 'deleted'; state: State<T> };
```

## Branded Types

```typescript
declare const __brand: unique symbol;

type Branded<T, B> = T & { [__brand]: B };

// Usage
type AgentId = Branded<string, 'AgentId'>;
type TaskId = Branded<string, 'TaskId'>;
type WorkflowId = Branded<string, 'WorkflowId'>;

// Factory functions
const AgentId = (id: string): AgentId => id as AgentId;
const TaskId = (id: string): TaskId => id as TaskId;
const WorkflowId = (id: string): WorkflowId => id as WorkflowId;

// Type guards
const isAgentId = (value: string): value is AgentId => 
  value.startsWith('agent-');

const isTaskId = (value: string): value is TaskId => 
  value.startsWith('task-');
```

## Validation with Zod

```typescript
import { z } from 'zod';

// Agent schema
const AgentSchema = z.object({
  id: z.string().startsWith('agent-'),
  type: z.enum(['coding', 'research', 'execution', 'general']),
  provider: z.enum(['openai', 'anthropic', 'google', 'ollama']),
  model: z.string(),
  config: z.object({
    temperature: z.number().min(0).max(2),
    maxTokens: z.number().positive(),
    timeout: z.number().positive(),
    retryPolicy: z.object({
      maxAttempts: z.number().positive(),
      backoff: z.enum(['fixed', 'exponential']),
      baseDelay: z.number().positive()
    }),
    tools: z.array(z.string())
  }),
  state: z.enum(['idle', 'running', 'paused', 'error', 'terminated'])
});

type Agent = z.infer<typeof AgentSchema>;

// Validation
const validateAgent = (data: unknown): Agent => {
  return AgentSchema.parse(data);
};

// Safe validation
const safeValidateAgent = (data: unknown): Result<Agent, ValidationError> => {
  const result = AgentSchema.safeParse(data);
  if (result.success) {
    return ok(result.data);
  } else {
    return err(new ValidationError(result.error));
  }
};
```

## Result Type

```typescript
type Result<T, E = Error> = 
  | { ok: true; value: T }
  | { ok: false; error: E };

const ok = <T>(value: T): Result<T, never> => ({ ok: true, value });
const err = <E>(error: E): Result<never, E> => ({ ok: false, error });

// Usage
const parseAgent = (json: string): Result<Agent, ParseError> => {
  try {
    const data = JSON.parse(json);
    return ok(validateAgent(data));
  } catch (e) {
    return err(new ParseError(e));
  }
};

// Result utilities
const map = <T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => U
): Result<U, E> => {
  if (result.ok) {
    return ok(fn(result.value));
  }
  return result;
};

const flatMap = <T, U, E>(
  result: Result<T, E>,
  fn: (value: T) => Result<U, E>
): Result<U, E> => {
  if (result.ok) {
    return fn(result.value);
  }
  return result;
};
```

## Type Utilities

```typescript
// Deep partial
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

// Deep readonly
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object ? DeepReadonly<T[P]> : T[P];
};

// Nullable properties
type Nullable<T> = { [P in keyof T]: T[P] | null };

// Non-nullable
type NonNull<T> = T extends null | undefined ? never : T;

// Strict omit
type StrictOmit<T, K extends keyof T> = Omit<T, K>;

// Strict pick
type StrictPick<T, K extends keyof T> = Pick<T, K>;

// Keys of type
type KeysOfType<T, U> = {
  [K in keyof T]: T[K] extends U ? K : never;
}[keyof T];

// Function properties
type FunctionPropertyNames<T> = {
  [K in keyof T]: T[K] extends Function ? K : never;
}[keyof T];

// Required by
type RequiredBy<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

// Partial by
type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;
```

## Quick Start

```typescript
import { 
  Agent, Task, Workflow,
  Event, Message, State,
  Branded, Result, ok, err 
} from 'phenotype-types';

// Define custom types
type MyAgentId = Branded<string, 'MyAgentId'>;
const MyAgentId = (id: string): MyAgentId => id as MyAgentId;

// Use core types
const agent: Agent = {
  id: 'agent-123',
  type: 'coding',
  provider: 'openai',
  model: 'gpt-4-turbo',
  config: {
    temperature: 0.7,
    maxTokens: 4096,
    timeout: 30000,
    retryPolicy: {
      maxAttempts: 3,
      backoff: 'exponential',
      baseDelay: 1000
    },
    tools: ['bash', 'read_file', 'write_file']
  },
  capabilities: ['code', 'debug', 'refactor'],
  state: 'idle'
};

// Validate
const result = safeValidateAgent(agent);
if (result.ok) {
  console.log('Valid agent:', result.value);
} else {
  console.error('Validation error:', result.error);
}
```

## References

- TypeScript Handbook: https://www.typescriptlang.org/docs/handbook/intro.html
- Zod: https://zod.dev/
- fp-ts: https://gcanti.github.io/fp-ts/
- io-ts: https://gcanti.github.io/io-ts/
- type-fest: https://github.com/sindresorhus/type-fest
- ts-pattern: https://github.com/gvergnaud/ts-pattern
