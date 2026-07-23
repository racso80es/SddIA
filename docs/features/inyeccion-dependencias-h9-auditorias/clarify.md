---
feature_name: inyeccion-dependencias-h9-auditorias
created: "2026-07-22"
purpose: Estabilización PBI-043 Hito 3 (H9) — auditorías residual con DI por capacidades (R6–R7 / AC-H9)
branch_name: feat/inyeccion-dependencias-h9-auditorias
persist_ref: docs/features/inyeccion-dependencias-h9-auditorias
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H9-AUDITORIAS
execution_id: c9e4b17a-6f2d-4a8e-9c3b-1d5e8f0a7b42
phase: closed-apto
agents: mayeuta,dedalo,tekton,argos
inventory_recount_date: "2026-07-23"
inventory_without_capability: 8
inventory_with_capability: 34
n_ola_floor: 5
taxonomy_baseline: "doc:closure | proc:git-sync | fs:persist | bus:route | qa:probe | audit:compliance | llm:interact"
taxonomy_version: "1.0.4"
bindings_baseline: "capability-bindings.md@1.3.0"
hito7_pr: "https://github.com/racso80es/SddIA/pull/144"
hito8_pr: "https://github.com/racso80es/SddIA/pull/147"
hito8_merge: "85052a868147ba04d8d045d232c968ba731aad9c"
r6_status: laudoed-rama-a
r6_dedalo_provisional: R6-A-qa-probe-tool-provider
ac_h9_branch: A
racso_countersign: "2026-07-23T06:53:00Z"
---

# Clarificación — PBI-043 Hito 3 (H9 · Auditorías)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md` (`document_id: PBI-043-DI-CATALOGO-RESIDUAL-H7`; `status: abierto`).
- **Ciclo:** feature `inyeccion-dependencias-h9-auditorias` · rama `feat/inyeccion-dependencias-h9-auditorias`.
- **Alcance declarado:** Hito 3 — **H9** · vectores **R6–R7**. Criterio **producto:** **AC-H9**. Regresión: suites DI MVP→H8 (`capability_di` / `cerbero_di`).
- **Precedente cerrado Hito 1 (H7):** `docs/features/inyeccion-dependencias-h7-nucleo-fs` — PR [#144](https://github.com/racso80es/SddIA/pull/144). Núcleo FS §3.1 (`fs:persist`).
- **Precedente cerrado Hito 2 (H8):** `docs/features/inyeccion-dependencias-h8-familia-route` — PR [#147](https://github.com/racso80es/SddIA/pull/147) merge `85052a8`. Alta `bus:route`; 3 routes DI.
- **Precedente cerrado Done PBI-042:** `docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md`.
- **Normas / SSOT (Cúmulo):** `capability-taxonomy.md` v1.0.3 (`doc:closure`, `proc:git-sync`, `fs:persist`, `bus:route`), `capability-bindings.md` v1.2.0, `capability_contracts`, `eda-coverage.json`, `evolution`.
- **Runtime intacto a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · `capability_di_output_validator` · orden `resolve → gate → rbac → envelope → inject → output_validator`.
- **Hecho runtime hoy:** gate/resolver/scan solo reconocen providers `skill:` | `action:` (`provider_fs_rel` / `provider_md_rel` / `scan_catalog_providers`). **`tool:` excluido.**
- **ED objetivo (§3.3):** 5 auditorías — 4 tool-bound + 1 solo `agent:argos`.
- **Fuera de alcance:** H10 gobernanza/interactores (§3.4); R10 EDA-only; reescritura amplia runtime DI; GesFer/F1; altas libres al Códice sin laudo; deuda PPR #136; archivo PBI-043 (queda en `pending/`).

## D1 — Matriz de validación (AC-INV · recuento start post-H8)

> Recuento 2026-07-22 sobre `SddIA/process/*.md` (excl. `process-contract`, `index`). Total ED process = 42. Drift vs PBI §3.3+§3.4 post-H8: **ninguno**.

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| H7 núcleo FS | **Hecho (main)** | PR #144 |
| H8 familia route | **Hecho (main)** | PR #147 merge `85052a8` |
| Runtime DI MVP→H8 | **Hecho (main)** | no reescribir salvo bug; extensión mínima `tool:` solo si laudo R6-A |
| Taxonomía 4 términos | **Hecho** | v1.0.3 + `bus:route` |
| Bindings v1.2.0 | **Hecho** | fila `bus:route` → `skill:bus-operator` |
| Process con `requires_capability` | **29** | baseline 26 + 3 §3.2 |
| Process **sin** `requires_capability` | **13** | §3.3 (5) + §3.4 (8) |
| §3.3 auditorías (5) | **Ausente DI** | tools chaos/audit + Argos |
| Providers `tool:` en DI | **No soportado** | solo `skill:`/`action:` |
| Aduana EDA genómica | **Preservar** | `orphan_count == 0` post-mutación |
| H10 / R10 | **Fuera** | no absorber en este `persist_ref` |

### Inventario H9 §3.3 — piso de ola (N_ola ≥ 5)

| ED | Delegados actuales | Capacidad | Notas estabilizadas |
|----|--------------------|-----------|---------------------|
| `audit-sandbox-isolation-rbac` | `agent:tekton` + `tool:sandbox-breacher` (+ `agent:argos` cert) | *TBD bajo R6* | Caos sandbox; tool-bound |
| `audit-telemetry-compliance-breach` | `agent:tekton` + `tool:schema-corruptor` (+ `agent:argos` cert) | *TBD bajo R6* | Caos schema; tool-bound |
| `audit-thermodynamic-toll-failsoft` | `agent:tekton` + `tool:io-choke` (+ `agent:argos` cert) | *TBD bajo R6* | Caos E/S; tool-bound |
| `event-bus-audit` | `tool:event-bus-audit` | *TBD bajo R6* | Auditoría empírica bus; tool-bound puro |
| `telemetry-compliance-audit` | `agent:argos` (**sin tool**) | *TBD bajo R6* | Cumplimiento termodinámico; orquestación Argos |

**Piso Mayeuta:** exactamente estas **5** ED §3.3. Umbral `N_ola ≥ 5` = piso = lista; no bajar sin laudo Racso. Elevación a §3.4 = **fuera** de H9.

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT9-SCOPE** | Este ciclo = **R6–R7** + **AC-H9** + **AC-INV** + regresión MVP→H8. H10, R10, GesFer, F1, PPR #136 = fuera. |
| **L-BASELINE-H8** | Baseline innegociable = taxonomía 4 términos (v1.0.3) + bindings v1.2.0 + runtime DI + 29 process homologados post-H8. H9 **añade** homologación §3.3 bajo modelo R6 o documenta defer; no reescribe runtime salvo extensión mínima laudoada. |
| **L-R7-FLOOR** | Piso de ola = **5** ED §3.3 listadas. `N_ola = 5` (default estabilizado). Dedalo no baja; elevar solo con laudo Racso y sin absorber H10. |
| **L-R6-GATE** | Modelo DI tool-bound = **decisión abierta** (PBI Q2 / R6). Candidata documental `qa:probe` (contrato `qa.probe`) es **provisional**, no catálogo. Alta taxonomía+binding+contrato+extensión `tool:` **solo** con laudo Racso explícito. |
| **L-NO-INVENT** | Prohibido inventar `capability_id` fuera del Códice vigente (**AC-NO-INVENT**). Sin laudo Racso: **no** mutar `capability-taxonomy` / `capability-bindings` / `capability_contracts` / runtime DI. |
| **L-DEFER-OK** | Si laudo Racso = **defer** (o ausencia de laudo para alta): documentar defer explícito; **no** anotar `requires_capability` inventado; AC-H9 se satisface por rama *defer documentado*; genoma de capacidades + runtime intactos. |
| **L-REUSE-GATE** | Reuso de capacidad vigente (`fs:persist` / `doc:closure` / `proc:git-sync` / `bus:route`) **solo** si Dedalo demuestra semántica + fases coherentes. Default Mayeuta+Dedalo: **improbable** — probes caos/audit ≠ persist/route/closure/git. |
| **L-RUNTIME-MIN** | Si R6-A: extensión **mínima** gate/resolver/`provider_fs_rel`/`provider_md_rel`/`scan_catalog` para `tool:` como provider. Prohibida reescritura amplia de cadena DI. |
| **L-R2-MUTATION** | Mutación genoma ED vía `entity-manager` + `Domain_Entity_Updated` + `SddIA/evolution/` (**AC-SEAL**). Prohibido forjar `{name}.md` a mano sin sello. |
| **L-R2-EDA** | Post-mutación: `orphan_count == 0` (**AC-ORPHAN**). |
| **L-R3-REG** | Regresión obligatoria suites `capability_di` / `cerbero_di` (MVP→H8). No reabrir diseño de cadena DI. |
| **L-RUNTIME-PRESERVE** | Gate, resolver, Cerbero RBAC, envelope y output validator **permanecen** (salvo patch mínimo R6-A). |
| **L-PBI-LOC** | PBI-043 permanece en `docs/todos/pending/`; `pbi_archived: false` en este ciclo. Done global = H7–H10 (R10 opcional). |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core / deuda delivery-close en este `persist_ref`. |
| **L-TEKTON-GATE** | **Tekton no materializa R6-A** sin countersign Racso. Sin laudo → Rama B (defer) o **blocked escalate**. |

## D3.bis — Laudo Racso 2026-07-23 (countersign)

| ID | Veredicto |
|----|-----------|
| H9-A | OK — alta `qa:probe` |
| H9-B | OK — extensión DI `tool:` |
| H9-C | OK — `provides` tools caos/audit |
| H9-D | **RECHAZADO** reuso `qa:probe` → alta `audit:compliance` |
| H10-A | APROBADO — `llm:interact` (consumidores ciclo fino) |
| H10-B | DEFER total §3.4 |

**AC-NO-INVENT desbloqueado** para `qa:probe`, `audit:compliance`, `llm:interact`. Materialización H9 = R6-A + corrección H9-D.

## D3 — Resolución provisional Dedalo (R6) — no materialización

> Mayeuta+Dedalo cierran el **marco** de R6. La **alta** al Códice exige laudo Racso. Sin countersign: no hay mutación genoma/runtime.

### R6 — opciones admisibles

| Rama | Descripción | Condición |
|------|-------------|-----------|
| **R6-A** | Alta K=1 `qa:probe` (`qa.probe`) + binding canónico + `provides` en tools chaos/audit + extensión mínima runtime para provider `tool:` | **Requiere laudo Racso** |
| **R6-B** | **Defer** documentado (AC-H9 vía defer); sin altas; sin patch runtime | Default operativo sin laudo / laudo=defer |
| **R6-C** | `delegates_to` tool + `requires_capability` de orquestación **sin** alta (reuso catálogo vigente) | Solo si prueba semántica+reuso (**L-REUSE-GATE**) — Dedalo: **improbable** |

### Dictamen Dedalo provisional (no laudo)

| Afirmación | Veredicto Dedalo |
|------------|------------------|
| ¿Reuso R6-C viable? | **No** — ninguna capacidad vigente cubre semántica de probe/caos/audit QA; forzar `bus:route`/`fs:persist` = entropía semántica. |
| ¿Preferencia arquitectónica? | **R6-A** — unifica 4/5 tool-bound + permite anotar la 5ª (`telemetry-compliance-audit`) con la misma capacidad de orquestación QA. |
| ¿Id candidata? | `qa:probe` / contrato `qa.probe` — **provisional**; Racso puede laudar id equivalente. |
| ¿Blast-radius runtime? | Acotado: ramas `tool:` en `provider_fs_rel` / `provider_md_rel` / `scan_catalog_providers` + coherencia gate `delegates_to` tool; **no** reescribir Cerbero/envelope/output_validator. |
| ¿Sin laudo Racso? | **Prohibido** materializar R6-A → **R6-B** o escalate (**L-TEKTON-GATE** / **L-NO-INVENT**). |

### Ambigüedades acotadas (post-laudo · Dedalo `spec` — no Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | ¿Modelo R6? (PBI Q2) | **(A)** Alta `qa:probe` + tool-provider — **requiere laudo Racso**; **(B)** Defer; **(C)** Reuso vigente solo con prueba | **L-R6-GATE** + **L-NO-INVENT** + **L-DEFER-OK** + **L-REUSE-GATE**. Sin laudo → **(B)** o espera; **prohibido** materializar (A) |
| **Q2** | Provider canónico binding si (A) | ¿Un tool representativo vs multi-`provides` en los 4 tools? ¿Skill wrapper? | Una fila bindings; coherencia scan; sin inventar providers |
| **Q3** | Densidad DI / `delegates_to` en las 5 | (A) `requires_capability` + conservar tools/agentes; (B) path ciego; (C) mixto documentado | Coherencia taxonomía+bindings; `telemetry-compliance-audit` sin tool |
| **Q4** | Forma `provides` en tools | Migrar legado `capabilities:` → bloque `provides` DI; alcance exactamente tools §3.3 | Sin reescritura masiva tools fuera de ola |
| **Q5** | Estrategia de lotes | (A) un PR/lote con las 5; (B) sub-olas en el mismo `persist_ref` | Blast-radius; Argos audita 5/5 o defer |
| **Q6** | Evidencia AC-H9 / sello / orphan / defer | (A) fixture + assert orphan; (B) auditoría Argos evolution + sellos; (C) si defer: artefacto documental en `persist_ref` | Reproducible; sin Shell IDE crudo como SSOT |
| **Q7** | Alcance smoke regresión | Suites `capability_di` / `cerbero_di` mínimas vs pack completo MVP→H8 | **L-R3-REG**; no romper baseline H8 |
| **Q8** | Extensión runtime `tool:` | Superficie exacta de patch (resolver+gate+scan) | **L-RUNTIME-MIN**; tests unitarios del patch |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-H9** | **Rama A:** 5/5 ED §3.3 con `requires_capability` coherente taxonomía+bindings bajo modelo R6; mutación vía entity-manager + sellos + evolution; orphan 0; runtime preservado (o patch mínimo laudoado). **Rama B:** laudo Racso de **defer** explícito (o ausencia de laudo para alta) documentado en `persist_ref`; genoma de capacidades **sin** altas inventadas; orphan 0 si hubo mutación ED | Diff = 5 auditorías DI **o** defer trazable; aduana EDA verde |
| **AC-INV** | Inventario recontado al start; drift documentado | Tabla D1; `with=29` / `without=13` |
| **AC-NO-INVENT** | Ningún `capability_id` fuera del catálogo sin laudo Racso | Diff taxonomía/bindings vacío salvo laudo (A) |

Regresión / sellos obligatorios:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes (MVP→H8) | R7 / PBI-043 |
| **AC-SEAL** | Sello `Domain_Entity_Updated` trazable vía entity-manager (si hay mutación ED) | AC global PBI-043 |
| **AC-ORPHAN** | `orphan_count == 0` post-ola (o post-noop si defer puro) | AC global PBI-043 |

## D5 — Veredicto

**ok** — Requisitos H9 estables. **R6 pendiente laudo Racso.** Dictamen Dedalo provisional = **R6-A** (`qa:probe` + tool-provider mínimo). Sin countersign: Tekton **no** materializa alta/runtime → **R6-B defer** o **blocked escalate** (**L-TEKTON-GATE**).

Handoff: Dedalo `spec.md` / `plan.md` **solo tras** (1) countersign Racso R6-A, o (2) mandato explícito Rama B defer.
