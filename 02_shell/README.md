# /02_shell — The Shell

> Translation between the external world and the Core.

---

## Purpose

This stratum contains the primary adapters: components that receive input from the external world — users, HTTP requests, CLI commands — and translate them into domain calls in `01_core`.

The Shell is the primary cleavage plane: it separates the conceptual stability of the Core from the variability of input channels.

---

## What belongs here

- UI Components (React, Vue, Svelte)
- REST API controllers and GraphQL resolvers
- CLI handlers
- Input validation (format and type, not business rules)
- Output projection (response transformation for the client)

---

## Dependency Rule

- **Can import**: `00_nucleo`, `01_core`
- **Forbidden**: `03_infra`, `04_wiring`, `_lab`

> The Shell never accesses infrastructure directly. `03_infra` dependencies are injected via `04_wiring`.

---

## Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-registration.md
 * @layer L2
 */
import { validateUser } from '../../01_core/domain/user';

// ✅ Correct — translates external input to Core logic
export function RegistrationController(req, res) {
  const result = validateUser(req.body);
  if (!result.valid) {
    return res.status(400).json({ errors: result.errors });
  }
  // continue...
}

// ❌ Wrong — direct access to infrastructure
// import { db } from '../../03_infra/persistence'; // FORBIDDEN
```

---

## AI Agents Protocol

- Validate all external input before passing it to the Core
- Do not generate `fetch`, `sql` calls, or file access in this stratum
- UI components should be as functional as possible — logic stays in the Core
