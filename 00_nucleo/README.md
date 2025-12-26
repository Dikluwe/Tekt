### 1. README.md (English)

# /00_nucleo — The Seed

> **The Source of Truth.** Ground zero of crystallization.

## Purpose

This directory contains the **genetic material** of the project: specifications, business rules, architecture decisions, and interface contracts.

---

## 💎 Mathematical Formalism ($\mathcal{L}_0$)

To ensure structural integrity and minimize entropy, the Nucleus follows these formal constraints:

* **Axiomatization**: Let $S$ be the set of Specifications $s \in \{specs, contracts, adr\}$.
* **The Nucleation Invariant**: The existence of any implementation file $c \in \{01, 02, 03, 04\}$ requires a prior mapping to a specification $s$.
$$\forall c \in C, \exists s \in S : P(c, s)$$

* **Infimum Property**: $00\_nucleo$ is the infimum of the dependency poset $(X, \le)$. No layer may exist below it.
* **Structural Isomorphism**: The implementation must be an isomorphic representation of the structure defined in the contracts.

---

## The Nucleation Lock

> [!CAUTION]
> **No specification here = No code can be written.**

Before ANY feature is implemented in layers `01_core` through `04_wiring`, a corresponding document MUST exist in this directory.

## Directory Structure

```
00_nucleo/
├── specs/           # Feature specifications
├── contracts/       # Interface contracts & types
└── adr/             # Architecture Decision Records
```

* **specs/**: Feature specifications, business rules, and user stories.
* **contracts/**: Interface contracts, TypeScript types, and API schemas.
* **adr/**: Architecture Decision Records (Format: `NNNN-title-with-dashes.md`).

## Rules

1. **Read-First**: AI must read this directory before any code generation to align with the "genetic material".
2. **Human Approval**: All specifications require manual validation to prevent "hallucinated" requirements.
3. **Traceability**: Every code file must trace back to a specific document in this directory.
4. **Immutability**: Approved specifications are immutable; changes require a new ADR ($\Delta s \iff \exists adr_{new}$).

## Template Links

* [ADR Template](./adr/template.md)
* [Spec Template](./specs/.template.md)
* [Contract Template](./contracts/.template.md)

---
