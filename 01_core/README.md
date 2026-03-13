# /01_core — The Core

> Pure logic. The deterministic heart of the system.

---

## Purpose

This stratum contains only essential domain logic: entities, business rules, pure algorithms. Nothing here depends on databases, network, file system, or any external state.

The Core is the most stable structural phase of the system. Code here exists independently of time, infrastructure technologies, and input channels.

---

## Absolute Constraint: Zero I/O

> Code in this directory **must not**:
> - Access databases or make network requests
> - Read or write files
> - Access the system clock or mutable global state
> - Import external libraries (only the language's standard library)

Any need for I/O must be expressed as an interface in `00_nucleo/contracts/` and implemented in `03_infra/`.

---

## What belongs here

- Domain entities and Value Objects
- Business rules validation
- Pure algorithms and transformations
- Interface definitions (not implementations)
- Domain errors

---

## Structure

```
01_core/
├── entities/     # Domain entities and Value Objects
├── domain/       # Business rules and pure services
└── algorithms/   # Algorithms and transformations
```

---

## Dependency Rule

- **Can import**: `00_nucleo` (to implement contracts and satisfy specs)
- **Forbidden**: `02_shell`, `03_infra`, `04_wiring`, `_lab`

---

## Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-validation.md
 * @layer L1
 */

// ✅ Correct — pure function, no external dependencies
export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// ❌ Wrong — external I/O violates stratum purity
// import axios from 'axios';
// export async function verifyEmail(email: string) {
//   return await axios.get(`/api/verify?email=${email}`);
// }
```
