# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Is

**Tekt** is a meta-architecture specification — a structural framework that other projects adopt to solve AI-assisted code generation producing amorphous, untraceable growth. It is not a deployable application. There are no build, lint, or test commands.

The core hypothesis: keeping structured prompts versioned inside the project (in `00_nucleo/`) and enforcing strict layer dependencies maintains structural coherence through prolonged AI-assisted evolution.

## The Five-Layer Lattice

```
L₄ (Wiring)   04_wiring/   Composition root, DI — imports ALL layers except Lab
L₂ (Shell)    02_shell/    Primary adapters, I/O translation — imports L₀, L₁
L₃ (Infra)    03_infra/    Persistence, external APIs — imports L₀, L₁
L₁ (Core)     01_core/     Pure logic, zero I/O — imports L₀ only
L₀ (Nucleus)  00_nucleo/   Prompts and ADRs only — no code whatsoever
Lab           lab/         Quarantine zone for experiments — imports L₀ only
```

**Gravity Law:** Dependencies flow downward only. No numbered layer imports from Lab. L₄ is the sole layer permitted to see all others.

**L₁ absolute constraint:** Zero I/O. No database, network, file system, system clock, or mutable global state.

**L₄ absolute constraint:** Near-zero logic. Any `if/else` for business rules here is a structural defect.

**Lab promotion rule:** Lab code cannot be directly moved to a main layer — it must be completely rewritten with a new prompt and tests.

## Mandatory Agent Protocol (Nucleation Lock)

Before writing ANY code in L₁–L₄:

1. Inspect `00_nucleo/prompts/` for an existing prompt for the component
2. If no prompt exists → **STOP. Do not write code.** Ask the developer to create the prompt first
3. Read the full prompt (context, constraints, criteria, revision history)
4. Generate code **and** tests simultaneously from the prompt
5. Log the revision in the prompt's history (date, reason, affected files)

A component without a corresponding prompt in L₀ is structurally illegitimate, even if functionally correct.

## Mandatory Lineage Header

Every generated file in L₁–L₄ must begin with:

```typescript
/**
 * Crystalline Lineage
 * @prompt 00_nucleo/prompts/<name>.md
 * @layer L<n>
 * @updated YYYY-MM-DD
 */
```

## Key Documents

- `README.md` — Complete architecture reference with dependency table and quick start
- `MANIFESTO.md` — Problem statement, hypothesis, and evolution modes (Epitaxy, Nucleation, Metamorphism)
- `00_nucleo/prompt/template.md` — Template for creating new component prompts
- `00_nucleo/adr/template.md` — Template for architectural decisions

## Agent Workflows

Workflow files live in `.agents/workflows/`. Invoke them for structured operations:

- `init-legado.md` — Initialize a legacy project for Tekt migration (quarantines legacy in read-only L₂₀)
- `gerar-spec.md` — Generate a new L₀ structured prompt for a component
- `integrar-legado.md` — Refactor and move a legacy file into the appropriate Tekt layer
- `auditar-spec.md` — Audit prompt quality and completeness
- `clivar-modulo.md` — Split a large module across appropriate layers
