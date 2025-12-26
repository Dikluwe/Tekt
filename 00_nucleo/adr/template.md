# ⚖️ ADR Template: Architectural Decision Record

> **State Transformation Log**: This document records a significant mutation in the project's topology, providing the rationale for the evolution of the Crystal.

---

## 💎 Formalism ($\mathcal{L}_{adr}$)

* **State Mutation**: Let $S_t$ be the current state of the architecture. An ADR defines the transition $\Delta$ such that:
$$S_{t+1} = S_t + \Delta$$
* **Axiomatic Consistency**: The decision must not violate the core invariants defined in `00_nucleo/README.md`.
* **Causality**: Every decision must be traceable to a specific context or constraint $C$.

---

## ADR-NNNN: [Title]

### Status

`PROPOSED` | `ACCEPTED` | `DEPRECATED` | `SUPERSEDED BY ADR-XXXX`

### Date

YYYY-MM-DD

---

## Context

> What is the specific issue or constraint motivating this change? Describe the high-entropy state we are trying to crystallize.

[Describe the context here]

---

## Decision

> What is the proposed change? Describe how this decision modifies the system's topology or rules.

[Describe the decision here]

---

## Consequences

> How does this mutation affect the project's gravity and technical debt?

### ✅ Positive (Entropy Reduction)

* [e.g., Simplifies  logic $L_1$]
* [e.g., Hardens  isolation $L_3$]

### ❌ Negative (Added Complexity)

* [e.g., Increases boilerplate in $L_4$]

### ⚙️ Neutral

* [List any trade-offs that don't directly impact structural integrity]

---

## Alternatives Considered

| Alternative | Pros | Cons |
| --- | --- | --- |
| **Option A** | [Brief Benefit] | [Major Drawback] |
| **Option B** | [Brief Benefit] | [Major Drawback] |

---

## References

* **Specification**: [Link to 00_nucleo/specs/file.md]
* **Discussion**: [Link to Issue/PR]
* **External**: [Relevant blog posts or papers]
