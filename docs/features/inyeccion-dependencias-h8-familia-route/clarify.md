---
feature_name: inyeccion-dependencias-h8-familia-route
created: "2026-07-22"
purpose: Estabilización PBI-043 Hito 2 (H8) — familia route residual con DI por capacidades (R4–R5 / AC-H8)
branch_name: feat/inyeccion-dependencias-h8-familia-route
persist_ref: docs/features/inyeccion-dependencias-h8-familia-route
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H8-FAMILIA-ROUTE
execution_id: a7c3e91f-2b84-4d6e-9f01-5c8a2e7d4b63
phase: mayeuta-stabilization
agents: mayeuta
inventory_recount_date: "2026-07-22"
inventory_without_capability: 16
inventory_with_capability: 26
n_ola_floor: 3
taxonomy_baseline: "doc:closure | proc:git-sync | fs:persist"
bindings_baseline: "capability-bindings.md@1.1.0"
hito7_pr: "https://github.com/racso80es/SddIA/pull/144"
hito7_merge: "8f882b82c74660e0ec5be8c0ed2931bfab454290"
---

# Clarificación — PBI-043 Hito 2 (H8 · Familia route)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md` (`document_id: PBI-043-DI-CATALOGO-RESIDUAL-H7`; `status: abierto`).
- **Ciclo:** feature `inyeccion-dependencias-h8-familia-route` · rama `feat/inyeccion-dependencias-h8-familia-route`.
- **Alcance declarado:** Hito 2 — **H8** · vectores **R4–R5**. Criterio **producto:** **AC-H8**. Regresión: suites DI MVP→H7 (`capability_di` / `cerbero_di`).
- **Precedente cerrado Hito 1 (H7):** `docs/features/inyeccion-dependencias-h7-nucleo-fs` — PR [#144](https://github.com/racso80es/SddIA/pull/144) merge `8f882b8`. Núcleo FS §3.1 homologado (`fs:persist`); runtime DI preservado.
- **Precedente cerrado Done PBI-042:** `docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md`.
- **Normas / SSOT (Cúmulo):** `capability-taxonomy.md` v1.0.2 (`doc:closure`, `proc:git-sync`, `fs:persist`), `capability-bindings.md` v1.1.0, `capability_contracts`, `eda-coverage.json`, `evolution`.
- **Runtime intacto a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · `capability_di_output_validator` · orden `resolve → gate → rbac → envelope → inject → output_validator`.
- **ED objetivo (§3.2):** `route-domain`, `route-orchestration`, `route-telemetry` — hoy `delegates_to: agent:cumulo`; **sin** `requires_capability`.
- **Referencia ya homologada:** `route-domain-event` → `fs:persist` (H7); revalidar drift, no reescribir salvo drift.
- **Fuera de alcance:** H9 auditorías (§3.3); H10 gobernanza/interactores (§3.4); R10 EDA-only; reescritura runtime DI; GesFer/F1; altas libres al Códice sin laudo; deuda PPR #136; archivo PBI-043 (queda en `pending/`).

## D1 — Matriz de validación (AC-INV · recuento start post-H7)

> Recuento 2026-07-22 sobre `SddIA/process/*.md` (excl. `process-contract`, `index`). Drift vs PBI §3.2+§3.3+§3.4 post-H7: **ninguno**.

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| H7 núcleo FS | **Hecho (main)** | PR #144 merge `8f882b8` |
| Runtime DI MVP→H7 | **Hecho (main)** | no reescribir salvo bug |
| Taxonomía 3 términos | **Hecho** | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings v1.1.0 | **Hecho** | sin fila `bus:route` |
| Process con `requires_capability` | **26** | baseline 18 + 8 §3.1 |
| Process **sin** `requires_capability` | **16** | §3.2 (3) + §3.3 (5) + §3.4 (8) |
| §3.2 familia route (3) | **Ausente DI** | solo `agent:cumulo` |
| `route-domain-event` | **Homologado H7** | `fs:persist` en fases FS |
| Aduana EDA genómica | **Preservar** | `orphan_count == 0` post-mutación |
| H9–H10 / R10 | **Fuera** | no absorber en este `persist_ref` |

### Inventario H8 §3.2 — piso de ola (N_ola ≥ 3)

| ED | Capacidad | Notas estabilizadas |
|----|-----------|---------------------|
| `route-domain` | *TBD bajo R4* | Fan-out bus fractal domain; `delegates_to: agent:cumulo` |
| `route-orchestration` | *TBD bajo R4* | Fan-out bus fractal orchestration; idem |
| `route-telemetry` | *TBD bajo R4* | Fan-out bus fractal telemetry; idem |

**Revalidación (no cuenta al piso salvo drift):**

| ED | Capacidad actual | Acción H8 |
|----|------------------|-----------|
| `route-domain-event` | `fs:persist` | Revalidar coherencia; mutar **solo si** drift vs H7 |

**Piso Mayeuta:** exactamente estas **3** ED §3.2. Umbral `N_ola ≥ 3` = piso = lista; no bajar sin laudo Racso. Elevación a §3.3/§3.4 = **fuera** de H8.

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT8-SCOPE** | Este ciclo = **R4–R5** + **AC-H8** + **AC-INV** + regresión MVP→H7. H9–H10, R10, GesFer, F1, PPR #136 = fuera. |
| **L-BASELINE-H7** | Baseline innegociable = taxonomía 3 términos + bindings v1.1.0 + runtime DI + 26 process homologados post-H7. H8 **añade** homologación §3.2 o documenta defer; no reescribe runtime salvo bug. |
| **L-R5-FLOOR** | Piso de ola = **3** ED §3.2 listadas. `N_ola = 3` (default estabilizado). Dedalo no baja; elevar solo con laudo Racso y sin absorber H9+. |
| **L-R4-Q1-GATE** | Capacidad de familia route = **decisión abierta** (PBI Q1). Candidata documental `bus:route` u equivalente es **provisional**, no catálogo. Alta taxonomía+binding+contrato **solo** con laudo Racso explícito. |
| **L-NO-INVENT** | Prohibido inventar `capability_id` fuera del Códice vigente (**AC-NO-INVENT**). Sin laudo Racso: **no** mutar `capability-taxonomy` / `capability-bindings` / `capability_contracts`. |
| **L-DEFER-OK** | Si laudo Racso = **defer**: documentar defer explícito en `spec.md`/`clarify` handoff; **no** anotar `requires_capability` inventado; AC-H8 se satisface por rama *defer documentado*; genoma de capacidades intacto. |
| **L-REUSE-GATE** | Reuso de capacidad vigente (`fs:persist` / `doc:closure` / `proc:git-sync`) **solo** si Dedalo demuestra semántica + fases coherentes. Default Mayeuta: **no** forzar `fs:persist` en fan-out puro `agent:cumulo` (evitar entropía semántica). |
| **L-RDE-REVAL** | `route-domain-event` ya H7: revalidar; tocar solo ante drift. No cuenta como unidad del piso N_ola=3. |
| **L-R2-MUTATION** | Mutación genoma ED vía `entity-manager` + `Domain_Entity_Updated` + `SddIA/evolution/` (**AC-SEAL**). Prohibido forjar `{name}.md` a mano sin sello. |
| **L-R2-EDA** | Post-mutación: `orphan_count == 0` (**AC-ORPHAN**). |
| **L-R3-REG** | Regresión obligatoria suites `capability_di` / `cerbero_di` (MVP→H7). No reabrir diseño de cadena DI. |
| **L-RUNTIME-PRESERVE** | Gate, resolver, Cerbero RBAC, envelope y output validator **permanecen**. |
| **L-PBI-LOC** | PBI-043 permanece en `docs/todos/pending/`; `pbi_archived: false` en este ciclo. Done global = H7–H10 (R10 opcional). |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core / deuda delivery-close en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | ¿Capacidad para familia route? (PBI Q1 / R4) | **(A)** Alta `bus:route` (u id laudo-equivalente) + taxonomía + binding + contrato — **requiere laudo Racso explícito**; **(B)** **Defer** documentado (AC-H8 vía defer; sin mutar Códice); **(C)** Reuso de capacidad vigente solo con prueba semántica Dedalo | **L-R4-Q1-GATE** + **L-NO-INVENT** + **L-DEFER-OK** + **L-REUSE-GATE**. Sin laudo Racso → default operativo = **(B)** o espera laudo; **prohibido** materializar (A) en genoma |
| **Q2** | Densidad DI / `delegates_to` en las 3 routes | (A) `requires_capability` + conservar `agent:cumulo`; (B) path ciego si la capacidad resuelve el fan-out; (C) mixto documentado | Coherencia taxonomía+bindings; sin inventar proveedores |
| **Q3** | Provider binding si (A) | ¿Qué artefacto canónico provee `bus:route`? (skill/action/agent) | Solo tras laudo Racso; fila única en bindings |
| **Q4** | Estrategia de lotes | (A) un PR/lote con las 3; (B) sub-olas en el mismo `persist_ref` | Blast-radius acotado; Argos audita 3/3 o defer |
| **Q5** | Evidencia AC-H8 / sello / orphan / defer | (A) fixture + assert orphan; (B) auditoría Argos evolution + sellos; (C) si defer: artefacto documental de laudo en `persist_ref` | Reproducible; sin Shell IDE crudo como SSOT |
| **Q6** | Alcance smoke regresión | Suites `capability_di` / `cerbero_di` mínimas vs pack completo MVP→H7 | **L-R3-REG**; no romper baseline H7 |
| **Q7** | ¿Tocar `provides` además de `requires_capability`? | (A) solo `requires_capability` (default); (B) `provides` si Dedalo identifica proveedor | Coherencia; sin altas sin laudo |
| **Q8** | Drift `route-domain-event` | (A) sin cambios; (B) remendar si drift vs H7 | **L-RDE-REVAL** |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-H8** | **Rama A:** 3/3 ED §3.2 con `requires_capability` coherente taxonomía+bindings bajo modelo R4; mutación vía entity-manager + sellos + evolution; orphan 0; runtime preservado. **Rama B:** laudo Racso de **defer** explícito documentado en `persist_ref`; genoma de capacidades **sin** altas inventadas; orphan 0 si hubo mutación ED | Diff = 3 routes DI **o** defer trazable; aduana EDA verde |
| **AC-INV** | Inventario recontado al start; drift documentado | Tabla D1; `with=26` / `without=16` |
| **AC-NO-INVENT** | Ningún `capability_id` fuera del catálogo sin laudo Racso | Diff taxonomía/bindings vacío salvo laudo (A) |

Regresión / sellos obligatorios:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes (MVP→H7) | R5 / PBI-043 |
| **AC-SEAL** | Sello `Domain_Entity_Updated` trazable vía entity-manager (si hay mutación ED) | AC global PBI-043 |
| **AC-ORPHAN** | `orphan_count == 0` post-ola (o post-noop si defer puro) | AC global PBI-043 |

## D5 — Veredicto

**ok** — Requisitos H8 estables. **Addendum Racso 2026-07-22T16:56:00Z:** laudo **Q1=(A)** alta `bus:route` autorizado. Tekton materializa Rama A (Códice + ola §3.2).
