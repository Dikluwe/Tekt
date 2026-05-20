# /_lab — The Arena

> Quarantine zone. Experiments live and die here.

---

## Purpose

This stratum plays two key roles in code science and system exploration:

1. **The Arena (v1.3 Role)**: A high-entropy space for exploratory code, prototypes, and short-term disposable spikes that do not yet satisfy the main system's invariants.
2. **The Workbench (v1.4 Role)**: A platform for controlled experiments involving hypotheses, controls, and measurements. It yields data and empirical evidence to support architectural decisions recorded in ADRs (e.g., measuring functional parity or performance comparison).

---

## Rules and Cost Regimes (v1.4)

Tekt v1.4 formalizes the **Lab regime** (low cost, high flexibility, and scientific focus) in contrast to the **System regime** (high cost, mandatory lineage, strict linting):

**Inside the Lab (`_lab/`):**
- **Cheap code**: Free development without the burden of maintaining full lineage through every phase of experimentation.
- **Optional prompt**: No rigid nucleation in L₀ is required for volatile experiments.
- **Free hypotheses**: Purity and isolation rules are relaxed for quick prototyping.
- **Mandatory measurement**: Required only when the experiment is used as architectural evidence within an ADR.

**Outside the Lab:**
- No main system stratum (`L₁` through `L₄`) can import from `_lab`.

---

## The Promotion Act (Lab → System)

Migrating a component from the Lab to the main system requires a formal **Promotion Act** (never a simple copy-paste):

1. **Guided Rewrite**: The component is rewritten from scratch starting from a complete prompt in `00_nucleo/` (Nucleation).
2. **Normalization**: Structural adaptation of the code to satisfy all invariants of the target stratum (`L₁` through `L₄`).
3. **Lineage and Tests**: Inclusion of simultaneously generated tests and the mandatory `@prompt` lineage header.

---

## Suggested Structure

```
_lab/
├── experiments/   # One-off experiments
├── spikes/        # Short-term technical investigations
└── prototypes/    # Functional prototypes for validation
```

---

## When to use the Arena

- Investigate if a library solves the problem before committing to it
- Validate an implementation hypothesis without contaminating the system
- Rapid prototyping to test with users
- Explore refactorings before executing them in the main system
