### 1. README.md (English Version)

# /02_shell — The Surface

> **The Crystal's Face.** Everything externally touchable.

## Purpose

This directory contains **Primary Adapters**: components that receive input from the outside world (Users, HTTP Requests, CLI Commands) and translate it into calls to the core domain.

---

## 💎 Mathematical Formalism ($\mathcal{L}_2$)

To maintain the integrity of the core while interacting with high-entropy environments, the Shell follows the **Translation Morphism**:

* **Interface Projection**: Let $W$ be the World (External Data) and $C$ be the Core Domain. The Shell is a mapping function $f: W \to C$.
* **Type Safety**: The shell must ensure that for every external input $x \in W$, the transformation $f(x)$ results in a valid domain object $y \in L_1$.
* **Decoupling Invariant**: The Shell is forbidden from interacting with the Infrastructure layer $L_3$.
$$\text{dep}(L_2) \cap L_3 = \emptyset$$
* **Composition**: The Shell interacts with $L_1$ but is composed by $L_4$ (Wiring).

---

## What Lives Here

* 🖥️ **UI Components**: React, Vue, Svelte, or Vanilla JS views.
* 🌐 **API Controllers**: REST endpoints and GraphQL resolvers.
* ⌨️ **CLI Interfaces**: Command-line argument handlers.

## Dependency Rules

> [!IMPORTANT]
> **CAN import**: `01_core` (to use domain logic).
> **CANNOT import**: `03_infra` (to prevent direct database/network coupling).

* ✅ `02_shell`  `01_core`
* ❌ `02_shell`  `03_infra` (Infrastructure must be injected via `04_wiring`).

## AI Protocol (Auditoria de Isomorfismo)

1. **Input Validation**: AI must ensure all external data is validated before touching .
2. **Stateless UI**: UI components should be as functional as possible, delegating logic to the Core.
3. **No Direct I/O**: AI must not generate `fetch` or `sql` calls inside this directory.

---

### Exemplo / Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-registration.md
 */

// ✅ CORRECT - Maps external interaction to Core logic
import { validateUser } from '../../01_core/domain/user-logic';

// ❌ WRONG - Direct Infrastructure access
// import { db } from '../../03_infra/persistence'; // FORBIDDEN!

export function RegistrationController(req, res) {
  // Logic flows from Shell to Core
  const isValid = validateUser(req.body);
  // Implementation continues...
}

```
