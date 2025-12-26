### 1. README.md (English Version)

# /_lab — The Arena

> **Controlled Chaos.** The quarantined experimentation zone.

## Purpose

This directory is a **sandbox** for experiments, benchmarks, and proofs of concept. Code here is volatile, high-entropy, and can be deleted at any time.

---

## 💎 Mathematical Formalism ($\mathcal{L}_{lab}$)

The Lab is defined as a **High Entropy Zone** where structural constraints are suspended to allow for exploration:

* **Entropy Maximization**: Unlike the Core, the Lab allows $H \to \max$. There are no invariants for code quality or I/O isolation.
* **Absolute Isolation**: The main system $S = \{L_0, L_1, L_2, L_3, L_4\}$ is a closed set relative to the Lab.
$$\text{dep}(S) \cap \text{Lab} = \emptyset$$
* **Darwinian Selection (Normalization)**: The transition from Lab ($x$) to Core ($x'$) is not a direct mapping but a **Normalization Function** $N(x) = x'$, where all technical debt is stripped and the logic is re-axiomatized.

---

## The Quarantine Rule

> [!CAUTION]
> **ABSOLUTE QUARANTINE**
> The main system (00-04) **NEVER** imports anything from `_lab`.

## Lifecycle: The Darwinian Path

1. **Experiment**: Write messy code to explore possibilities.
2. **Benchmark**: Measure and compare different approaches mathematically.
3. **Decide**: Select the most efficient approach (The Winner).
4. **Crystallize**: **REWRITE FROM SCRATCH** in `01_core`. Direct copy-pasting is a violation of structural integrity.
5. **Delete**: Purge the experiment to maintain a clean lattice.

---

## Freedom Zone (The "No-Lock" Zone)

This is the ONLY place where the **Nucleation Lock** is suspended:

* ✅ AI can generate code without a prior spec in `00_nucleo`.
* ✅ I/O operations, external libraries, and hardcoded values are allowed.
* ✅ Messy, non-standard code is encouraged for speed.

---
