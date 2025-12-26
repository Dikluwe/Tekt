Este é um passo fundamental para resolver o problema da "explosão de contexto" em IAs. Ao refatorar o **ADR-0002** para o padrão apenas em inglês e com fundamentação matemática, transformamos uma técnica de organização em uma **Lei de Navegação Topológica**.

Aqui está a versão refatorada seguindo o padrão da **Crystalline Architecture**:

---

# ⚖️ ADR-0002: Fractal Context Mapping Strategy

> **State Transformation Log**: This document defines the mechanism for "Lazy Loading" project context, enabling AI agents to navigate high-complexity lattices without exceeding token budget or increasing noise.

---

## 💎 Formalism ($\mathcal{L}_{adr}$)

* **Context Optimization**: Let $T$ be the total tokens in the project and $C_{win}$ be the AI's context window. This strategy ensures the relevant context $c$ satisfies $c \ll C_{win}$ by using a fractal projection.
* **Structural Resolution**: The project map is a function $f: G \to M$ where $G$ is the file graph and $M$ is the set of maps. The resolution of $M$ increases as the agent moves deeper into the directory tree.
* **Integrity Invariant**: Every map is a derivative artifact; manual state changes in $M$ are forbidden to prevent "Map-Reality" divergence.

---

## Status

`ACCEPTED`

## Date

2025-12-12

---

## Context

Large Language Models (LLMs) have finite context windows. In complex lattices like this project, providing a full recursive file tree introduces significant **Structural Noise**. This noise degrades the AI's reasoning capability by forcing it to process irrelevant metadata, leading to high token costs and lower precision.

Human navigation relies on **Levels of Detail (LoD)**—viewing global structures before zooming into local details. The current system lacks this "Lazy Loading" capability.

---

## Decision

We authorize the implementation of the **Fractal Mapping** system, managed by the `tools/cartographer.rs` automation.

1. **Global Atlas (`PROJECT_MAP.md`)**: An architectural overview located at the root. It maps ONLY the top-level layers ($L_0$ to $L_4$) and their direct children.
2. **Local Maps (`_MAP.md`)**: Module-specific maps located in component subdirectories. These maps contain a granular file list with one-line descriptions extracted from **Magic Comments** (`//!` or `#`).
3. **Automated Cartography**: Maps are strictly read-only artifacts. They are regenerated via `cartographer.rs` during Git Hooks or pre-commit protocols.

---

## Consequences

### ✅ Positive (Entropy Reduction)

* **Token Efficiency**: Context loading is now $O(\log N)$ instead of $O(N)$ for large trees.
* **Reduced Hallucination**: Agents only see files relevant to the current layer, minimizing context contamination.
* **Automated Topology**: The project structure becomes self-documenting through code.

### ❌ Negative (Added Complexity)

* **Morphism Maintenance**: Developers MUST maintain magic comments in the first line of every file.
* **Build-time Overhead**: Adds a minor compilation step to update the Atlas during development.

### ⚙️ Neutral

* **Read-Only Constraint**: Any manual edit to a `_MAP.md` file will be overwritten, requiring a change in mindset from "manual documentation" to "source-of-truth documentation".

---

## References

* **Tool Specification**: [[link suspeito removido]]
* **Core Principle**: [Lineage Tracing]
* **Previous ADR**: [[ADR-0001: Adopt Crystalline Architecture](https://www.google.com/search?q=../adr/0001-adopt-crystalline.md)]

---
