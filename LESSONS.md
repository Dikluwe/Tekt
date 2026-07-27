# Lessons for Tekt v1.4

**State**: ALIVE — grows in parallel with the steps of typst-crystalline.

**Origin**: distillation of methodological inversions discovered during the refactoring of typst (typst-crystalline) based on the Tekt Manifesto v1.3. From L7 onwards, also from sibling projects (crystalline-lint, lente/tekt-cargo-dsm).

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

**Note (2026-07)**: the A/B protocol has an additional property discovered later — it is also a **spec-completeness oracle**. If Agent B cannot generate tests from L₀ alone, the prompt is underspecified. The nucleation lock gains an audit mechanism.

---

## L7 — Compression: the scarce resource is context, not code

**v1.3 assumed**: the problem is amorphous growth; structure is the answer. Structure appears as an end — preserved because it is the invariant.

**Empirical discovery**: in three independent projects (typst-crystalline, crystalline-lint, lente), structure revealed itself as a means to a more precise end: making the right context fit the agent's decision. The lente distills the sentence the Manifesto lacks: *compress the program into a form that fits the decision*. The prompt is the loadable unit addressed by component; the minimum report is compression of the execution; the ADR is compression of the decision; the DSM is compression of form; step granularity (M1) is compression of work.

**Candidate form for v1.4**: the **Compression** (or Addressing) principle — the lattice is a context-addressing scheme for statistical agents. Structure exists so that, at each decision, exactly the relevant context is loadable — no more, no less. M1, L5, and agent readability derive from this principle; they do not precede it.

**Living evidence**: lente README ("compress the program into a form that fits the decision"; "For AI agents" section — JSON as compressed, verifiable context); minimum report hypothesis (L5). External corroboration: ICM (arXiv:2603.16021) applies the same principle to runtime context delivery, with per-layer token budgets.

---

## L8 — Verification stack: four layers, four failure classes

**v1.3 said**: two verifiers — the linter (structure) and the human (fidelity). L2 introduced the executable oracle in brownfield; the complete verification architecture remained unformalized.

**Empirical discovery**: actual verification operates in four layers with distinct costs and cadences, each catching a failure class the others miss:

| Layer | What it verifies | Failure class | Cadence |
|-------|------------------|---------------|---------|
| lint | legal form | structural violation | continuous (CI) |
| tests | declared spec | divergence from prompt | per nucleation |
| oracle | fidelity to substrate | divergence from original | per milestone |
| human | judgment | what none of the three reach | per ADR |

And the oracle has two habitats that practice used without distinguishing: the **legacy substrate** (the entire original system, read-only) and the **oracle corpus** (curated, minimal fixtures for differential measurement).

**Candidate form for v1.4**: explicitly declare the stack, with cadence and failure class per layer. The executable oracle becomes a lattice citizen — not a brownfield addendum — with both habitats named: **legacy substrate** (L₂₀) and **oracle corpus** (lab/ or fixtures).

**Living evidence**: `lab/parity/` (typst-crystalline); `oraculo/biteproof/` and fixtures v01–v14 (crystalline-lint); `init-legado.md` workflow (read-only L₂₀).

---

## L9 — Fine gravity: the total order within strata

**v1.3 said**: gravity between strata (L₄ → L₀). About intra-stratum dependencies, silence.

**Empirical discovery**: practice numbered slices within L1 — the lente has `05_investiga`, `06_resolve`, `07_filtro`, `08_ranking`, `09_estrutura`, all L1, with gravity among them (investiga → resolve). Strata are coarse equivalence classes of a finer stability order. And intra-layer cycles emerge when semantics requires iteration: typst's eval↔layout introspection would be a cycle between phases if the `engine` container did not own the loop.

**Candidate form for v1.4**: **fine gravity** — directory numbering expresses stability order also within the stratum, verifiable by upper-triangular DSM. Intra-stratum vocabulary:

- **phase** — unit of transformation (parse, eval, layout)
- **container** — module that owns its children's orchestration and transforms nothing; **cycle-breaker**: the parent iterates so that children need not import each other
- **facade** — the container seen from outside: a single name for the closure, entry point for consumers

**Living evidence**: lente structure (`01_core` + `05`–`09`, all L1); engine/export separation in typst-crystalline (the container as closure of the introspection fixpoint).

---

## L10 — Vocabulary drift: three concepts, three resolutions, no name

**v1.3 assumed**: the lattice vocabulary (L₀–L₄, lab) suffices to name what practice encounters.

**Empirical discovery**: each project resolved unnamed concepts its own way: reports ("L5" in the lente README vs. `00_nucleo/lessons/` in practice), oracle (`lab/typst-original/` vs. the workflows' L₂₀ vs. the linter's `oraculo/`), intra-layer structure (engine phases in typst vs. numbered slices in the lente). Three projects, three independent resolutions — a vocabulary-gap pattern, not indiscipline. An unnamed concept gets resolved locally, and each resolution diverges.

**Candidate form for v1.4**: vocabulary consolidation — (a) L₀ has **three artifact classes by causal position**: prompt (ex-ante), report (ex-post), ADR (transversal); the "L5" label dies; (b) legacy substrate ≠ oracle corpus (see L8); (c) phase/container/facade as intra-stratum grammar (see L9).

**Living evidence**: lente README ("L5 reports" vs. reports in `00_nucleo/lessons/`); Tekt CLAUDE.md (`init-legado` → L₂₀); crystalline-lint structure (`oraculo/` at repo root).

---

## L11 — Attested claims: the oracle layer needs a receipt, not an adjective

**v1.3/L8 assumed**: the oracle layer (L8) verifies "fidelity to substrate" — implicitly, this means someone runs a comparison and reports the result. The report format was left open.

**Empirical discovery**: in typst-crystalline's math-layout front (P885–P917), reports that skipped the measurement produced closed steps that weren't actually fixed. "Delimiters now scale adequately with content" (P912) closed the step, and it took two more rounds (P916, P917) insisting on the exact `mutool trace` table already used by earlier steps in the same front (P901, P905, P911) before the claim was re-measured and found still false for the common case (simple fractions, 2×2 matrices — the glyph stayed fixed; only tall content happened to work, via a different, unrelated mechanism). The failure mode was not dishonesty — it was that "measured" and "asserted" produced textually indistinguishable reports. Nothing in the report format forced the distinction.

**Candidate form for v1.4**: borrow the *Attested Computation* triad from the Open Knowledge Format (OKF v0.2, §10) as a required shape for any oracle-layer claim: **executor** (the exact command/tool that produces the measurement — not "measured", but `mutool trace <file>`), **receipt** (the exact fields the executor must return — glyph_id, advance, position, not "grew correctly"), **attester** (a deterministic comparison of receipt against the target, pass/fail — not "seems right"). A report closing an oracle-layer claim without all three is *unattested*, and an unattested claim cannot close a step, however confident the prose. This refines L8, it does not replace it: the oracle layer already existed; what was missing was the shape that separates a claim from a proof.

**Living evidence**: typst-crystalline P901/P905/P911 (attested — `mutool trace` tables with real glyph IDs and positions, before and after); P912/P913/P914 (unattested — prose claims, no receipt, later found incomplete); P916/P917 (retroactive attestation, forced by review, which is what actually found the remaining bug). External source: Open Knowledge Format v0.2 §10 (`GoogleCloudPlatform/knowledge-catalog`), independently converging on the same executor/receipt/attester shape for agent-maintained knowledge corpora.

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

**Note (2026-07)**: M4 gained vocabulary in L9 (phase / container / facade) and mechanism (fine gravity). The migration to v1.4 should merge the two entries.

### M5 — Typed steps

M1 asked for the division law; M2 observed diagnostic-first. The mature form: **the step has a type**. Diagnostic, archeology, nucleation, reconciliation, and epitaxy have different minimum reports and different verifications. The division law M1 asked for emerges from type: divide when the type changes, consolidate when the type repeats. Each type declares its minimum report and its verification — diagnostic verifies by inventory, nucleation by tests, reconciliation by isomorphism, epitaxy by diff containment.

### M6 — Onomastic drift

In brownfield, the agent renames during regeneration (tacit name "improvements"), cutting the join key with the oracle: coverage starts marking false-absent where a port exists, and phantom-new where a rename exists. Candidate rule: **in brownfield, names are inheritance, not decision** — the substrate's name is the default; any other name is a divergence and requires an ADR. Reconciliation mechanics: resolved inventory by graph (not text), structural matching (seed of exact matches + propagation by edge neighborhood), and rename verification by graph isomorphism modulo renamed ids. Evidence: ongoing name corrections in typst-crystalline; lente/tekt-cargo-dsm as the inventory and verification instrument.

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
- Fine gravity and container — lente README (L1 slices `05`–`09`); engine/export separation (typst-crystalline)
- Verification stack — `oraculo/biteproof/` and fixtures v01–v14 (crystalline-lint); `init-legado.md` workflow (L₂₀)
- Onomastic drift — name corrections in typst-crystalline; reconciliation map (to be produced with lente)
- Attested claims — typst-crystalline P901/P905/P911 (attested) vs. P912/P913/P914 (unattested, later corrected by P916/P917); Open Knowledge Format v0.2 §10, `GoogleCloudPlatform/knowledge-catalog/okf/SPEC.md`
