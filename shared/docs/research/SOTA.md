# SOTA: Shared Utility Libraries Landscape

## State of the Art: Shared UI Utility Libraries

### Research Date

2026-04-05

### Context

This document analyzes the shared utility libraries landscape relevant to phenotype-shared, which provides TypeScript/React utilities including UI components, hooks, stores, API clients, and configuration helpers.

---

## 1. Shared Utility Libraries Landscape

### 1.1 Component Libraries

#### Radix UI + shadcn/ui Pattern

The dominant pattern for modern React component libraries:

| Library | Approach | Size | Accessibility | Customization |
|---------|----------|------|---------------|---------------|
| **Radix UI** | Headless primitives | ~50KB | WCAG 2.1 AAA | Full control |
| **Headless UI** (Tailwind) | Headless primitives | ~40KB | WCAG 2.1 AA | Full control |
| **Chakra UI** | Styled components | ~80KB | Partial | Theme system |
| **Material UI** | Styled components | ~150KB | Partial | Limited |
| **Ant Design** | Styled components | ~250KB | Partial | Limited |

**Phenotype Choice**: Radix UI + custom components

Rationale:
- Headless approach provides maximum flexibility
- shadcn/ui demonstrated production viability
- Full control over styling via Tailwind
- Smaller bundle than alternative solutions

#### shadcn/ui Specific Advantages

1. **Copy-paste over dependency**: Components live in your repo
2. **Full customization**: No override prop hell
3. **No import mystery**: Easy to debug
4. **Automatic updates**: CLI handles upgrades

### 1.2 Hook Libraries

#### Popular Hook Collections

| Library | Hooks Count | Bundle Size | TypeScript | React 18/19 |
|---------|-------------|-------------|------------|--------------|
| **react-use** | 50+ | ~25KB | Yes | Yes |
| **ahooks** | 40+ | ~30KB | Yes | Yes |
| **react-query** (TanStack) | 5 core | ~15KB | Yes | Yes |
| **Zustand** | Store only | ~5KB | Yes | Yes |

**Phenotype Choice**: Custom hooks + react-use patterns

Rationale:
- React-use patterns well-understood and battle-tested
- Custom hooks for domain-specific needs (deployment, metrics, logs)
- Smaller than ahooks for our use case
- First-class TypeScript support

#### Hook Pattern Evolution

```typescript
// Old pattern: Options object
const { data, loading, error } = useAsync(async () => {
  return fetchUsers()
}, [])

// Modern pattern: AbortController + cleanup
const [state, setState] = useState<T>(initial)
const abortRef = useRef<AbortController>()

useEffect(() => {
  const controller = new AbortController()
  abortRef.current = controller
  
  fetch(url, { signal: controller.signal })
    .then(setState)
    .catch(handleError)
  
  return () => controller.abort()
}, [url])
```

### 1.3 State Management

#### Zustand vs Alternatives

| Library | Bundle | Boilerplate | Persistence | DevTools |
|---------|--------|-------------|-------------|----------|
| **Zustand** | ~5KB | Minimal | Middleware | Yes |
| **Jotai** | ~8KB | Minimal | Middleware | Yes |
| **Redux Toolkit** | ~25KB | Moderate | Middleware | Yes |
| **Valtio** | ~10KB | Minimal | Proxies | Yes |
| **Recoil** | ~30KB | High | Built-in | Yes |

**Phenotype Choice**: Zustand

Rationale:
- Minimal boilerplate compared to Redux
- Excellent TypeScript support
- Persist middleware for localStorage
- No provider wrapper needed
- Smaller bundle than Jotai

### 1.4 API Clients

#### HTTP Client Comparison

| Client | Bundle | Interceptors | TypeScript | Retry | Caching |
|--------|--------|-------------|------------|-------|---------|
| **fetch** | 0KB | Manual | Yes | Manual | Manual |
| **axios** | ~15KB | Yes | Yes | Plugin | Plugin |
| **ky** | ~4KB | Yes | Yes | Extension | Extension |
| **ofetch** | ~10KB | Yes | Yes | Extension | Built-in |
| **tRPC** | ~30KB | Yes | Yes | Built-in | Built-in |

**Phenotype Choice**: Custom fetch-based client

Rationale:
- Zero additional bundle size (fetch built-in)
- Full control over interceptor implementation
- Matches our TypeScript patterns exactly
- No abstraction leakage

#### Interceptor Pattern Implementation

```typescript
// Our pattern: Pipeline-based interceptors
class ApiClient {
  private interceptors = {
    request: [] as Interceptor[],
    response: [] as Interceptor[],
    error: [] as Interceptor[],
  }
  
  async request<T>(url: string, config: RequestConfig): Promise<T> {
    let ctx = { url, config }
    
    // Request pipeline
    for (const fn of this.interceptors.request) {
      ctx = await fn(ctx)
    }
    
    const response = await fetch(ctx.url, ctx.config)
    
    // Response pipeline
    let result = response
    for (const fn of this.interceptors.response) {
      result = await fn(result)
    }
    
    return result
  }
}
```

### 1.5 Utility Functions

#### Class Merging Solutions

| Utility | Bundle | Conflicts | Tailwind |
|---------|--------|----------|----------|
| **clsx** | ~1KB | No | No |
| **tailwind-merge** | ~2KB | Yes | Yes |
| **cn()** | ~3KB | Yes | Yes |
| **classnames** | ~1KB | No | No |

**Phenotype Choice**: `cn()` = clsx + tailwind-merge

```typescript
// Our implementation
import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs))
}

// Usage
cn('px-2 py-1', isActive && 'bg-blue-500', { 'text-white': darkMode })
// Returns merged classes without Tailwind conflicts
```

---

## 2. Comparison with Other Approaches

### 2.1 Monorepo Structure Alternatives

#### Single Package vs Multi-Package

| Approach | Pros | Cons |
|----------|------|------|
| **Single @phenotype/shared** | Simple import, one version | All-or-nothing consumers |
| **Multi-package (current)** | Granular deps, tree-shaking | More complex publishing |
| **Virtual packages** | Flexible, no duplication | Complex tooling |

**Current Decision**: Multi-package with independent versioning

#### Workspace vs Single Repo

| Approach | Pros | Cons |
|----------|------|------|
| **npm/yarn workspaces** | Native, fast | Limited tooling |
| **pnpm workspaces** | Fast, strict | Different semantics |
| **Turborepo** | Caching, parallelism | Additional dependency |
| **Single repo (current)** | Simple, Git-native | No workspace features |

### 2.2 Export Strategy Comparison

```typescript
// CommonJS style
module.exports = { Button, Dialog }

// ESM default
export default { Button, Dialog }

// ESM named (chosen)
export { Button, Dialog }

// ESM with subpath (chosen)
export { Button } from './components/button'
export type { ButtonProps } from './components/button'
```

### 2.3 Build Tool Comparison

| Tool | Speed | Output | DTS | Config |
|------|-------|--------|-----|--------|
| **tsc** | Fast | CJS/ESM | tsc | tsconfig.json |
| **tsup** | Very fast | CJS/ESM | Yes | Minimal |
| **Rollup** | Fast | Any | rollup-plugin-dts | Complex |
| **esbuild** | Fastest | ESM only | No | Minimal |

**Current Decision**: tsc for ui-* packages, tsup for authkit-config

---

## 3. Novel Patterns in phenotype-shared

### 3.1 Interceptor Pipeline Architecture

The API client implements a **pipeline interceptor pattern** not commonly seen in lightweight clients:

```typescript
// Each phase is a separate interceptor array
interceptors: {
  request: Interceptor[]   // Transform before fetch
  response: Interceptor[] // Transform after fetch
  error: Interceptor[]     // Handle errors globally
}

// Returns unsubscribe function
const unsubscribe = client.addRequestInterceptor(async (ctx) => {
  // Add auth header
  return { ...ctx, headers: { ...ctx.headers, Authorization: token }}
})

// Later: clean up
unsubscribe()
```

**Novel Aspect**: Self-cleaning interceptors via closure return

### 3.2 Store Slice Pattern with TypeScript

Zustand stores use explicit state/actions separation with full TypeScript inference:

```typescript
// State interface
interface DeploymentState {
  deployments: Deployment[]
  selectedDeploymentId: string | null
  isLoading: boolean
}

// Actions interface
interface DeploymentActions {
  setDeployments: (deployments: Deployment[]) => void
  addDeployment: (deployment: Deployment) => void
  // ...
}

// Combined type
export type DeploymentStore = DeploymentState & DeploymentActions
```

**Novel Aspect**: Exported compound types for external consumers

### 3.3 Hook Return Type Derivation

Hooks derive return types from options:

```typescript
interface UseApiOptions<T> {
  url: string
  method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH"
  initialData?: T
  onSuccess?: (data: T) => void
  onError?: (error: Error) => void
}

interface UseApiResult<T> {
  data: T | undefined
  error: Error | null
  isLoading: boolean
  isError: boolean
  refetch: () => Promise<void>
}

// Return type derived from options
export function useApi<T>(options: UseApiOptions<T>): UseApiResult<T>
```

**Novel Aspect**: Generic type flows through entire hook

### 3.4 AbortController Management Pattern

React hooks properly manage AbortController lifecycle:

```typescript
const abortControllerRef = useRef<AbortController | null>(null)

useEffect(() => {
  // Create new controller for each effect run
  const controller = new AbortController()
  abortControllerRef.current = controller
  
  // Cleanup: abort on unmount or dependency change
  return () => {
    controller.abort()
  }
}, [/* dependencies */])
```

**Novel Aspect**: Proper cleanup prevents race conditions

### 3.5 Persist Partialization

Zustand persist middleware with partialization for storage limits:

```typescript
persist(
  (set, get) => ({ /* state and actions */ }),
  {
    name: "deployment-store",
    partialize: (state) => ({
      // Only persist limited data
      deployments: state.deployments.slice(0, 50),
      selectedDeploymentId: state.selectedDeploymentId,
    }),
  }
)
```

**Novel Aspect**: Storage-aware partialization

### 3.6 CVA with Slot Composition

Combining Class Variance Authority with Radix Slot for polymorphic components:

```typescript
import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"

const buttonVariants = cva("btn-base", { variants: { ... } })

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button"
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    )
  }
)
```

**Novel Aspect**: Polymorphic variants without styled-components

---

## 4. Emerging Trends

### 4.1 Headless Components

Moving toward fully headless components with composition:

```typescript
// Future pattern
<Dialog>
  <Dialog.Trigger>Open</Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Title>Title</Dialog.Title>
    <Dialog.Description>Description</Dialog.Description>
  </Dialog.Content>
</Dialog>
```

### 4.2 Fine-Grained React 18/19 Patterns

```typescript
// Concurrent mode safe
const [isPending, startTransition] = useTransition()

startTransition(() => {
  setState(newValue)
})
```

### 4.3 Server Components

```typescript
// Client component boundary
'use client'

export async function ServerComponent() {
  // Server-side data fetching
  const data = await db.query()
  return <ClientComponent initial={data} />
}
```

### 4.4 Signals-Based Reactivity

Potential future migration to signals:

```typescript
// Instead of useState
const count = signal(0)
const doubled = computed(() => count.value * 2)
```

---

## 5. Recommendations

### 5.1 Immediate Actions

1. **Add Storybook** for component documentation
2. **Increase test coverage** for hooks and stores
3. **Add Visual Regression** testing with Playwright

### 5.2 Medium-Term

1. **Migrate to pnpm** for faster installs and strict deps
2. **Add Turborepo** for build caching
3. **Create Figma specs** for all components

### 5.3 Long-Term

1. **Explore SolidJS** compatibility layer
2. **Evaluate Qwik** patterns for resumability
3. **Consider WASM** for performance-critical utilities

---

## References

- [Radix UI Primitives](https://www.radix-ui.com/)
- [shadcn/ui](https://ui.shadcn.com/)
- [Zustand](https://zustand-demo.pmnd.rs/)
- [react-use](https://github.com/streamich/react-use)
- [TanStack Query](https://tanstack.com/query)
- [Class Variance Authority](https://cva.style/)
- [Tailwind Merge](https://github.com/cast Stay tuned)
