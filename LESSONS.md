# Lessons for Tekt v1.4

**State**: ALIVE — grows in parallel with the steps of typst-crystalline.

**Origin**: distillation of methodological inversions discovered during the refactoring of typst (typst-crystalline) based on the Tekt Manifesto v1.3.

**Function**: to gather, in pure form and independent of typst, lessons that should enter the Manifesto v1.4. Each entry is a candidate for a Principle, Mechanics, or Pattern.

**Inclusion Rule**: only entries that represent a **genuine inversion** compared to v1.3 should be added here — not detailed refinements or assertions that the Manifesto already covers implicitly. Refinements live in local ADRs.

**Form Rule**: each lesson is described in three short blocks:
- **v1.3 said (or assumed)** — what was stated, explicitly or implicitly
- **Empirical discovery** — what the project showed was insufficient or wrong
- **Candidate form for v1.4** — concrete proposal for a principle, mechanics, or vocabulary

The long discussion remains in the original ADRs and reports; this is the distillation.

---

## L1 — Brownfield mode: the inverted causal arrow

**v1.3 assumed**: prompt before code. Greenfield implicit throughout the Manifesto. Nucleation is the starting point; code is the materialization.

**Empirical discovery**: in refactoring, code comes before the prompt. Nucleation cannot be the first act — it must be preceded by a **reverse inference** operation, where existing code is read to reconstruct the L₀ it *would have had* if it were greenfield. Without this step, the agent regenerates in the dark.

**Candidate form for v1.4**: distinguish two operational modes of Tekt.

- **Greenfield mode** — prompt → code. Canonical Tekt v1.3.
- **Brownfield mode** — code → inference → prompt → regenerated code.

Brownfield mode introduces a new phase before nucleation, currently without a canonical name (candidates: *exegesis*, *archeology*, *exhumation*). This phase produces an intermediate artifact that serves as raw material for the prompt.

**Living evidence**: typst-crystalline operates entirely in brownfield mode. The vanilla typst in `lab/typst-original/` is the substrate; each nucleation step was preceded by reading the corresponding vanilla code.

---

## L2 — External oracle: executable reference as a second source of truth

**v1.3 said**: the prompt is the truth. The linter verifies structure. The human verifies fidelity. There is no third source.

**Empirical discovery**: in brownfield, there is an **executable reference** — the original system — against which the fidelity of the regenerated code can be measured. This reference does not replace human judgment, but serves as a **partial oracle**: it allows automated observational parity, declared divergence with justification, and graded status of "implemented" instead of binary exists/does-not-exist.

**Candidate form for v1.4**: introduce the concept of a **fidelity oracle**.

- In greenfield, the oracle is only the prompt + human judgment.
- In brownfield, there is a second oracle: the source system or a regression corpus.
- The executable oracle allows **measurement** where previously there was only argumentation.

Parity vocabulary that emerged and deserves formalization:

- `implemented` — feature present, without reservations
- `implemented⁺` — feature present with documented approximation
- `partial` — capture exists; consumption divergent or absent
- `absent` — not captured
- `scope-out` — explicit ADR declares it out of scope

**Living evidence**: ADR-0033 (vanilla functional parity), ADR-0054 (graded observational profile), ADR-0075 (vanilla integration), `lab/parity/`.

---

## L3 — Lab as a workbench: code science

**v1.3 said**: `_lab` is a disposable arena. Volatile code, without lineage, isolated from the main system. Migration from the arena to the system requires a complete rewrite.

**Empirical discovery**: the lab has a more ambitious role than a spike arena — it can be a **platform for comparative experiments** where one implementation is proven to be better than another by measurable criteria. This introduces a source of truth into Tekt that v1.3 did not foresee: **empirical evidence as an architectural argument**, alongside the written argument in an ADR.

**Candidate form for v1.4**: lab now has two distinct roles.

- **Arena** (v1.3 role) — experimental code without lineage, disposable.
- **Workbench** (new role) — controlled experiments with hypothesis, control, measurement. Produces **results** that can substantiate architectural decisions.

The workbench allows an ADR to be substantiated not only by written reasoning, but by a reproducible experiment registered in L_lab. This brings Tekt closer to the applied scientific method.

**Open question**: what criteria separate a workbench experiment from a decision ready for promotion? Probably a **proof gradient** with declared stages.

**Living evidence**: typst-crystalline itself operates as a workbench for Tekt — `lab/parity/` measures parity vs. vanilla with a controlled corpus and dated reports.

---

## L4 — Two cost regimes: system vs. lab

**v1.3 said**: implicitly, all code is expensive. The prompt is expensive. Lineage discipline is high in all strata.

**Empirical discovery**: working with AI, the cost of generating code is radically asymmetric compared to the cost of generating a prompt. Disposable exploratory code is **cheap**. Forcing the same discipline regime on code that will exist for hours and code that will exist for years is wasteful.

**Candidate form for v1.4**: explicitly declare **two cost regimes** with distinct disciplines.

- **System regime** — `00_nucleo` through `04_wiring`. Expensive code, expensive prompt, mandatory lineage, strict linter.
- **Lab regime** — `_lab/`. Cheap code, optional prompt, free hypotheses, mandatory measurement only when the experiment is used as evidence.

And between the two, a **one-way flow**: lab → system requires an **act of promotion** — rewriting with a complete prompt, in layers, with lineage, and tests. System → lab is free (extracting to experiment).

**Open question**: does the act of promotion need formal criteria? Probably yes — otherwise the boundary erodes.

---

## L5 — Prospective report: leaving it readable on rewind

**v1.3 said**: the prompt is the cause, the code is the result. The generation cycle does not produce another artifact.

**Empirical discovery**: actual generation involves **discoveries along the way** — tacit choices made by the agent, limitations found, alternatives that proved redundant (e.g., discovering that a proposed structure already existed, or that a divergence from the spec was unnecessary). Without recording these discoveries, future readers cannot distinguish what was decided based on the prompt from what was discovered during execution.

**Candidate form for v1.4**: each nucleation produces three artifacts, not two.

- **Prompt** in L₀ — cause.
- **Code + tests** in the corresponding strata — materialization.
- **Execution report** — short record of what happened between the prompt and the result: discoveries, alternatives rejected in-flight, tacit decisions, divergences from the spec.

The report is **prospective**: written now to be read by whoever comes later. It does not replace the prompt; it complements it.

**Open critical question**: what is the minimum viable report? typst-crystalline accumulated extensive reports (consolidated reports, pre-implementation diagnostics, inventories) that grow beyond the original prompt. Finding the **minimum viable report** is the optimization point declared by the author — what preserves prospective readability without inflating the documentation layer.

**Hypothesis to test**: the minimum viable report has 3 sections and fits on half a page: *what the prompt requested*, *what was delivered*, *discoveries along the way*.

---

## L6 — Isolated generation: combating context leakage

**v1.3 assumed**: The same agent can generate code and tests simultaneously in the same cycle from the specification, without structural or informational bias.

**Empirical discovery**: The same agent generating both suffers from **context leakage**. The AI agent generates tests biased by the implementation it just wrote, testing the "coincidence" and short-cuts of its own implementation rather than testing the L₀ Spec in a strict, agnostic manner.

**Candidate form for v1.4**: Assign **two autonomous and isolated agents** for the materialization phase:

- **Agent A (Implementer)**: Receives the L₀ Spec and exclusively generates the code in the corresponding strata.
- **Agent B (Tester)**: Receives exclusively the L₀ Spec and generates the tests independently, without seeing the code generated by Agent A in any way.

The success of the cleavage is validated when the union of both agents' materializations compiles and passes the test suite under black-box isolation.

---

## Secondary inversions — candidates for Mechanics, not Principle

The lessons above reformulate Tekt at the Principles level. There are also finer observations, candidates for auxiliary Mechanics:

### M1 — Step granularity as a typed decision

The "step" axis is not uniform. typst-crystalline empirically discovered that steps compose into sub-steps (A/B/C/D/E/F/G) when the real work is non-uniform, and that this granularity is a **decision**, not a detail. v1.4 can formalize a division law: when to divide, when to consolidate, when to aggregate in a closed series.

### M2 — Diagnostic-first as a pattern

Recurring pattern: before prompt, inventory; before inventory, diagnostic. Appeared as the "8th application of the diagnostic-first pattern" in a specific series. In brownfield it is almost mandatory; in greenfield it can be optional.

### M3 — ADR lifecycle is not monotonic

v1.3 treats ADRs as permanent decisions. typst-crystalline showed three states that v1.3 did not foresee:

- **superseded-by** — ADR replaced by a newer one
- **accepted structurally** — accepted by force of events, before formal declaration
- **accepted retroactively** — last empirical condition closes after structural declaration

v1.4 needs to formalize this lifecycle.

### M4 — Phase patterns within L₁

Some architectural decisions are not about layers, but about **phases within a layer** — mutable reading during walk vs. immutable reading post-walk, trackable sub-stores, sealing points. v1.4 can catalog patterns of "distinct phases within the same stratum" without promoting them to new strata.

---

## Room to grow

This list grows. Every time something methodological surprises us, it is registered here in distilled form. The entry criterion is simple: **if v1.3 did not foresee it, and the project showed it was needed, it is a candidate for v1.4**.

When an entry matures enough, it migrates to the next Manifesto. The document empties as v1.4 absorbs it.

---

## Cross-references

- Tekt Manifesto v1.3 — `D:/Git/Tekt/MANIFESTO.pt.md`
- Vanilla divergence ADRs — ADR-0026, ADR-0033, ADR-0054, ADR-0075
- Diagnostic-first — series P154A, P156B, P185A, P192A, P200A, P204A, P205A
- ADR lifecycle — ADR-0066 (superseded), ADR-0073 (retroactive acceptance), ADR-0074 (final acceptance)
- Observational parity — `lab/parity/`, consolidated report P206D
