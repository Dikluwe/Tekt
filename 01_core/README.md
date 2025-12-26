### 1. README.md (English Version)

# /01_core — Pure Crystal

> **Platonic Logic.** The heart of pure business rules.

## Purpose

This directory contains **pure domain logic**: entities, algorithms, mathematical functions, and business rules with **absolutely no I/O**.

---

## 💎 Mathematical Formalism ($\mathcal{L}_1$)

To ensure a deterministic core, this layer is defined as a collection of **Pure Morphisms**:

* **Purity ($\mathcal{P}$)**: Every function $f$ in $L_1$ must be a pure function.
$$\forall x \in X, \forall t \in T : f(x, t) = f(x)$$
(The output depends solely on the input, independent of the system time  or external state).
* **Side-Effect Isolation**: The set of side effects $\mathcal{E}$ for any operation in this layer must be empty.
$$\text{SideEffects}(L_1) = \emptyset$$
* **Stateless Determinism**: For any state $S$ and input $I$, the transition function $\delta$ must be a deterministic mapping: $\delta: S \times I \to S'$.

---

## The Zero I/O Rule

> [!CAUTION]
> **ABSOLUTE RESTRICTION**
> Code in this directory **MUST NOT**:
> * Access databases or network requests.
> * Read/write files or access the system clock.
> * Import external libraries (except the language standard library).
> 
> 

## Allowed

✅ **Pure functions** and immutable data structures.
✅ **Domain Entities** and Business Rule validation.
✅ **Interfaces** (Abstract definitions) for external dependencies.
✅ **Mathematical algorithms** and stateless computations.

## Directory Structure

```
01_core/
├── entities/        # Domain entities (Models, Value Objects)
├── algorithms/      # Pure algorithms (Math, Sorting, Transforms)
└── domain/          # Business rules (Validators, Pure Services)

```

## Dependency Rule

* **Can Import**: `00_nucleo` (to implement contracts and specs).
* **Forbidden**: `02_shell`, `03_infra`, `04_wiring`, `_lab`.

---

/**
 * Crystalline Lineage
 * @spec 00_nucleo/specs/user-validation.md
 */

// ✅ CORRECT - Pure Morphism f: string -> boolean
export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

// ❌ WRONG - Side Effect Violation (External I/O)
// import { db } from '../03_infra/db';

---
