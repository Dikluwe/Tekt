# /_lab — The Arena

> Quarantine zone. Experiments live and die here.

---

## Purpose

This stratum exists for exploratory code, prototypes, and experiments that do not yet satisfy the main system's invariants. It is a deliberately isolated high-entropy space.

---

## Rules

**Inside the Arena:**
- Code doesn't need full lineage
- Purity and isolation rules are relaxed
- Experiments can reference `00_nucleo` specs for guidance
- No constraints on I/O or external dependencies

**Outside the Arena:**
- No main system stratum can import from `_lab`
- Arena code is not directly promoted to the main system

---

## Migration to Main System

For Arena code to migrate to `01_core`, `02_shell`, or `03_infra`, it must be **entirely rewritten** satisfying:

1. Traceable lineage to `00_nucleo`
2. All dependency rules of the target stratum
3. `@spec` header pointing to an existing document

Copying and pasting from the Arena to the main system is a structural violation. Rewriting is mandatory.

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
