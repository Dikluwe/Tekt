### 1. README.md (English Version)

# /03_infra — The Foundation

> **The Crystal's Support.** Controlled side effects and external persistence.

## Purpose

This directory contains **Secondary Adapters**: implementations of the interfaces defined in the Core. It handles the "dirty" work of interacting with the physical world (Databases, External APIs, File Systems).

---

## 💎 Mathematical Formalism ($\mathcal{L}_3$)

Unlike the Core, the Infrastructure layer acknowledges the existence of state and side effects, but subjects them to **Interface Realization**:

* **Effect Realization**: Let $I \subset L_1$ be a set of abstract interfaces. Infrastructure provides a set of concrete implementations $M$ such that there is a realization morphism $r: M \to I$.
* **Side-Effect Encapsulation**: Let $\mathcal{E}$ be the set of side effects. While $\text{SideEffects}(L_3) \neq \emptyset$, these effects must be contained within the boundaries of the implementation, never leaking their internal types to $L_1$.
* **Dependency Inversion**: $L_3$  depends on $L_1$ to know *what* to implement, but $L_1$ never knows *how* $L_3$ works.
$$L_3 \implies L_1$$

---

## What Lives Here

* 🗄️ **Repositories**: SQL/NoSQL database implementations.
* ☁️ **External Clients**: SDKs for AWS, Stripe, SendGrid, etc.
* 📂 **Storage**: File system drivers.
* 📡 **Gateways**: Wrappers for external microservices.

## Dependency Rules

> [!IMPORTANT]
> **CAN import**: `01_core` (to implement its interfaces).
> **CANNOT import**: `02_shell`, `04_wiring`.

* ✅ `03_infra`  `01_core` (Implementing Domain Interfaces)
* ❌ `03_infra`  `02_shell` (Infrastructure never talks to the UI)

## AI Protocol (Isomorphism Audit)

1. **Contract Adherence**: AI must verify that every class in $L_3$ strictly follows an interface from $L_1$ or $L_0$.
2. **Implementation Only**: This layer should contain logic for "how to talk to the tool," not "business logic."
3. **Error Translation**: AI must map infrastructure errors (HTTP 500, SQL Timeout) to Domain Errors defined in $L_1$.

---

### Exemplo / Example

```typescript
/**
 * Crystalline Lineage
 * @spec 00_nucleo/contracts/user-repository.md
 */

// ✅ CORRECT - Implements an interface from Core
import { UserRepository } from '../../01_core/domain/interfaces';
import { Database } from './db-client';

export class SqlUserRepository implements UserRepository {
  // Logic restricted to data persistence
  async save(user) {
    return await Database.insert('users', user);
  }
}

// ❌ WRONG - Business logic leaked into infra
// if (user.age < 18) throw Error(); // THIS BELONGS IN 01_CORE!

```
