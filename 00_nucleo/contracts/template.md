# 📜 Contract Template

> **Morphism Definition**: This document defines the formal interface (contract) that bridges the abstract logic of the Core with its physical implementations.

---

## 💎 Formalism ($\mathcal{L}_{contract}$)

* **Contract Identity**: Let $I$ be the interface defined herein.
* **Morphism Mapping**: $\forall m \in M$ (Implementations),  must satisfy the type signature of $I$.
* **Nucleation Link**: This contract is a realization of the specification: `[[link-to-spec-in-00_nucleo]]`.

---

## Purpose

> Define concisely what this interface abstracts and the specific problem it solves within the domain.

---

## Definition

```typescript
/**
 * Interface Name: I[EntityName][Action]
 * Lineage: [00_nucleo/specs/filename.md]
 */
export interface IExampleInterface {
  /**
   * Method Purpose: [Brief description]
   * @param input - [Domain Type]
   * @returns [Morphism Result]
   */
  methodName(input: InputType): Promise<OutputType>;
}

```

---

## Implementations

| Layer | Class/Implementation | Purpose | Status |
| --- | --- | --- | --- |
| `03_infra` | `Sql[Name]Repository` | Production persistence (PostgreSQL) | Pending |
| `_lab` | `Mock[Name]Provider` | High-entropy testing sandbox | Active |

---

## Usage & Wiring

To maintain the **Composition Root** ($L_4$) integrity:

1. **Define** the interface in `01_core`.
2. **Implement** the logic in `03_infra`.
3. **Compose** (Wire) in `04_wiring`.

```typescript
// Wiring Example (04_wiring/index.ts)
const implementation = new SqlImplementation();
const controller = new Controller(implementation as IExampleInterface);

```

---

## Notes

* [Any specific invariant or edge case this contract must handle]
