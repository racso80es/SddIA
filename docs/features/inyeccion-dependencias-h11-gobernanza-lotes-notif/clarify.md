---
feature_name: inyeccion-dependencias-h11-gobernanza-lotes-notif
created: "2026-07-23"
purpose: Estabilización PBI-045 Hito 11 — DI gobernanza, lotes, suites, memoria y canales (N_ola=7)
branch_name: feat/inyeccion-dependencias-h11-gobernanza-lotes-notif
persist_ref: docs/features/inyeccion-dependencias-h11-gobernanza-lotes-notif
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md
document_id: PBI-045-DI-GOBERNANZA-LOTES-NOTIFICACIONES
execution_id: 881f8cf6-6a4c-48aa-9f76-d84df5641db8
phase: blueprint-design
agents: mayeuta,dedalo
dedalo_partition: "H11-A reuse qa/llm | H11-B reuse fs:persist | H11-C gov:rbac gated | H11-D channel:ingest gated"
inventory_recount_date: "2026-07-23"
inventory_without_capability: 7
inventory_with_capability: 35
n_ola_floor: 7
taxonomy_baseline: "doc:closure | proc:git-sync | fs:persist | bus:route | qa:probe | audit:compliance | llm:interact"
taxonomy_version: "1.0.4"
bindings_baseline: "capability-bindings.md@1.3.0"
spawn_laudo: "2026-07-23 Filtro C — H10-B rechazado; spawn PBI-045"
---

# Clarificación — PBI-045 Hito 11 (Gobernanza / Lotes / Notificaciones)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md` (`document_id: PBI-045-DI-GOBERNANZA-LOTES-NOTIFICACIONES`; `status: abierto`).
- **Ciclo:** feature `inyeccion-dependencias-h11-gobernanza-lotes-notif` · rama `feat/inyeccion-dependencias-h11-gobernanza-lotes-notif`.
- **Init:** `execute-process --process feature` → `workspace-init` **executed** (`execution_id: 881f8cf6-6a4c-48aa-9f76-d84df5641db8`). Fase Mayeuta runtime Kalma2 abortada (cursor-agent colgado); estabilización materializada en IDE (relay).
- **Alcance declarado:** Hito 11 — homologar **7** ED residuales. Criterio producto: **AC-H11**. Regresión: suites DI MVP→H10-A.
- **Precedente:** PBI-043 Done (Laudo Filtro C 2026-07-23) — 35/7; H10-B **no** materializado; spawn este PBI.
- **Normas / SSOT:** `capability-taxonomy.md` v1.0.4, `capability-bindings.md` v1.3.0, `features-documentation-pattern`, `external-ai-constraints`, `eda-coverage.json`.
- **Runtime a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · `capability_di_output_validator`.
- **Fuera:** reescritura runtime DI; reabrir PBI-043; GesFer/F1; PPR #136.

## D1 — Matriz de validación (AC-INV · recount 2026-07-23)

> Recuento sobre `SddIA/process/*.md` (excl. `process-contract`, `index`). Total ED process = 42. Drift vs PBI-045 §2: **ninguno**.

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| PBI-043 Done 35/7 | **Hecho** | PBI done + laudo Filtro C |
| Taxonomía 7 términos | **Hecho** | v1.0.4 |
| Bindings v1.3.0 | **Hecho** | fila `llm:interact` incluida |
| Process con `requires_capability` | **35** | recount |
| Process **sin** `requires_capability` | **7** | ≡ §2 PBI-045 |
| Runtime DI MVP→H10-A | **Preservar** | no reescribir salvo bug/laudo |
| Aduana EDA genómica | **Preservar** | `orphan_count == 0` post-mutación |

### Inventario H11 — piso de ola (N_ola ≥ 7)

| ED | Delegados actuales | Notas estabilizadas |
|----|--------------------|---------------------|
| `cerbero-governance-react` | `agent:cerbero` | Gobernanza RBAC reactiva Self-Healing |
| `radamanto-batch` | `agent:radamanto` | Lote telemetría + Self-Healing |
| `memory-evolution-ingest` | `agent:cumulo` | Ingesta vectorial evolution |
| `execute-suite` | multi + `action:execute-process` | Orquestación Suite Caos |
| `telegram-gateway` | `tool:telegram-gateway` | Canal aferente; **artefacto tool no localizado** bajo `directories.tools` en recount (Entropía — Dedalo) |
| `telegram-fallback-responder` | `agent:mayeuta` + `tool:send-telegram-notification` | Canal + síntesis; `send-telegram-notification` **sin** `provides` DI hoy |
| `capsule-invoke-smoke` | `tool:io-choke` | Smoke; `io-choke` ya `provides: qa:probe` |

**Piso Mayeuta:** exactamente estas **7** ED. `N_ola = 7`. Dedalo no baja sin laudo Racso.

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT11-SCOPE** | Este ciclo = **AC-H11** + **AC-INV** + **AC-THERMO** + **AC-NO-INVENT** + **AC-SEAL** + **AC-ORPHAN** + **AC-REG**. Sin absorber GesFer/F1/PPR#136. |
| **L-BASELINE** | Baseline innegociable = taxonomía 7 términos (v1.0.4) + bindings v1.3.0 + runtime DI + 35 process homologados. H11 **añade** homologación de las 7 o documenta defer por sub-ola. |
| **L-R1-FLOOR** | Piso de ola = **7** ED listadas. No bajar sin laudo Racso. |
| **L-THERMO** | Partición en sub-olas es **obligación de Dedalo** si blast-radius (heterogeneidad agent/tool/suite). Prohibido pack de 5 altas ortogonales en un ciclo. |
| **L-NO-INVENT** | Sin altas Códice/bindings/runtime sin countersign Racso. Reuso (`qa:probe`, `llm:interact`, `audit:compliance`, `fs:persist`, …) solo con prueba semántica Dedalo. |
| **L-REUSE-HINT** | Candidatos **provisionales** (no diseño ejecutables): `capsule-invoke-smoke`→`qa:probe` (io-choke ya provides); fases LLM de `telegram-fallback-responder`→`llm:interact`. Resto = *TBD Dedalo* (posible alta o reuso). |
| **L-TEKTON-GATE** | Tekton no inventa capacidad ni patch runtime sin countersign cuando Dedalo lo exija. |
| **L-PBI-LOC** | PBI-045 permanece en `pending/` hasta Done de este feature. |

## D3 — Ambigüedades abiertas (Dedalo / Racso)

| Q | Pregunta | Default Mayeuta |
|---|----------|-----------------|
| **Q1** | ¿Partición sub-olas canónica? | Dedalo propone; sugerencia no vinculante: (A) gobernanza+batch+memory · (B) suite · (C) telegram pair · (D) smoke |
| **Q2** | ¿Reuso `qa:probe` en `capsule-invoke-smoke`? | Preferible si semántica Caos/sonda coherente (H9) |
| **Q3** | ¿`telegram-gateway` tool fósil / path real? | Dedalo localiza o declara Entropía + plan de sello |
| **Q4** | ¿Alta capacidad canal (`notify:` / `channel:`) o reuso? | Default = no inventar; laudo Racso si hace falta |
| **Q5** | ¿`execute-suite` DI por fase o por process-level? | Dedalo; preferir mínimo blast |
| **Q6** | ¿`cerbero-governance-react` ↔ `audit:compliance`? | Probable **no** (gobernanza RBAC ≠ compliance auditor); Dedalo prueba |
| **Q7** | ¿Umbral Done = 7/7 o ≥ piso con defer documentado? | PBI: homologación umbral Dedalo ≥ piso; defer por sub-ola OK si laudoado |

## D4 — Resolución Dedalo (Q1–Q7)

| Q | Veredicto |
|---|-----------|
| Q1 | Sub-olas **A** (qa/llm) · **B** (`fs:persist`) · **C** (`gov:rbac`\|defer) · **D** (`channel:ingest`\|defer + forge tool.md) |
| Q2 | Reuso `qa:probe` en smoke **APROBADO** |
| Q3 | Entropía tool.md confirmada; H11-D |
| Q4 | Sin inventar en A/B; `channel:ingest` solo con Racso |
| Q5 | Suite: DI solo fase Resolución → `fs:persist` |
| Q6 | Reuso `audit:compliance` en Cerbero **RECHAZADO** |
| Q7 | Done ≥ piso con defer C/D laudoado OK; preferencia alta K=2 si Racso acepta |

## D5 — Handoff

| Actor | Acción |
|-------|--------|
| **Tekton** | H11-A + H11-B inmediato |
| **Racso** | Countersign H11-C / H11-D |
| **Tekton** | C/D laudoados → regresión |
| **Argos** | `validacion.md` |

**Veredicto Mayeuta+Dedalo:** blueprint **estable**. Tekton GO en A+B; C/D gated.
