---
feature_name: inyeccion-dependencias-migracion-catalogo
created: "2026-07-22"
purpose: Estabilización Hito 5 PBI-042 — sellado EDA Domain_Entity_Updated + ola migración catálogo ED (R11–R12; R13 opcional)
branch_name: feat/inyeccion-dependencias-migracion-catalogo
persist_ref: docs/features/inyeccion-dependencias-migracion-catalogo
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-MIGRACION-CATALOGO
execution_id: a8f4c2e1-6b9d-4e3a-9c7f-1d2e5a8b0c4f
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-042 Hito 5 (migración catálogo ED + sellado EDA)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.2.0; Hito 4 entregado PR #136 merge `6b0e98c`).
- **Ciclo:** feature `inyeccion-dependencias-migracion-catalogo` · rama `feat/inyeccion-dependencias-migracion-catalogo`.
- **Alcance declarado:** Hito 5 — **R11**, **R12**; **R13** opcional acotado. Criterios **producto:** **AC-R11**, **AC-R12**. Regresión: AC-R1/R2 (H2), AC-R5/R6/R7/R8 (H3), AC-R9/R10 (H4), AC-P1/P2/P3 (MVP).
- **Precedente cerrado Hito 4:** `docs/features/inyeccion-dependencias-envelope-homologacion` — `cerbero_di_envelope`, 8 ED homologadas, **L-R10-SEAL** diferido (sello `Domain_Entity_Updated` vía `entity-manager` no materializado en mutaciones R10).
- **Precedente cerrado Hito 3:** `docs/features/inyeccion-dependencias-gobernanza-asincronia` — `cerbero_di_rbac`, piloto EDA `CapabilityDi_*`, `proc:git-sync`, `capability_di_output_validator`.
- **Precedente cerrado Hito 2:** `docs/features/inyeccion-dependencias-resolucion-ciega` — `capability_di_resolver`, `di_binding` v2, `capability-bindings.md`.
- **Precedente MVP:** `docs/features/inyeccion-dependencias-capacidades` — `capability_di_gate`, taxonomía, DLQ.
- **Remisión explícita finalize Hito 4:** residual «Sello Domain_Entity_Updated (L-R10-SEAL)» + «Migración masiva catálogo ED» → este ciclo (**R11**, **R12**).
- **Normas / SSOT (Cúmulo):** `capability-taxonomy.md`, `capability-bindings.md` (`capability_di.bindings`), `eda-coverage.json`, `event_bus`, `capability_contracts`, `evolution`.
- **Runtime intacto a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · orden `resolve → gate → rbac → envelope → inject → output_validator`.
- **Fuera de alcance:** GesFer / Paciente 0; Fractura Core F1; archivo PBI-042 padre (L-PBI-LOC); sustitución total sync→EDA-only salvo laudo Racso.

## D1 — Matriz de validación (residual × estado post-Hito 4)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Envelope Cerbero `di_binding` (R9 / AC-R9) | **Hecho (H4)** | `cerbero_di_envelope.rs`; PR #136 |
| Homologación piloto ampliado (≥8 ED) (R10 / AC-R10) | **Hecho (H4)** | 8 ED: `feature`, `bug-fix`, `filesystem-manager`, `git-manager`, `refactorization`, `delivery-close-cycle`, `accept-pr`, `pull-request-review` |
| Mutación R10 vía integridad hash + verify | **Hecho (H4)** | `hash_signature` recalc + `verify-process-integrity` OK |
| Sello CRUD `Domain_Entity_Updated` en mutaciones DI homologadas | **Ausente / diferido** | L-R10-SEAL; finalize Hito 4 → backlog; AC-R11 |
| Migración catálogo ED más allá del piloto de 8 | **Ausente** | R12; H4 laudeó «no barrido masivo» |
| Taxonomía vigente (2 términos) | **Hecho (H3)** | `doc:closure`, `proc:git-sync` en `capability-taxonomy.md` |
| Binding table (2 filas) | **Hecho (H2/H3)** | `capability-bindings.md` |
| Piloto EDA DI async (R6 / AC-R6) | **Hecho (H3)** | `CapabilityDi_*` + `SDDIA_DI_EDA_PILOT`; sync H2 default |
| Ampliación piloto EDA DI | **Opcional** | R13; no sustituye path sync |
| Orden canónico DI + envelope | **Hecho (H4)** | Cadena post-RBAC pre-inject intacta |
| Aduana EDA genómica (`orphan_count == 0`) | **Hecho (H4 gate)** | `eda-coverage.json`; no implica sello CRUD por mutación R10 |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT5-SCOPE** | Este ciclo = **R11** + **R12** + AC-R11/AC-R12 producto + regresión AC-R1/R2, AC-R5/R6/R7/R8, AC-R9/R10, AC-P1/P2/P3. **R13** solo si Dedalo/Tekton demuestran valor medible acotado; no bloquea Done de R11/R12. GesFer, F1 y EDA-only total quedan fuera. |
| **L-BASELINE-8** | Baseline innegociable post-H4 = **8 ED** homologadas (§D1). R12 añade ED **nuevas** respecto a ese conjunto; no recontar las 8 como progreso de ola. |
| **L-R11-SEAL** | Cierra L-R10-SEAL: toda mutación genómica de ED en alcance R12 (y backfill opcional H4 si Dedalo lo exige en Q1) **debe** pasar por `entity-manager` con emisión CRUD de `Domain_Entity_Updated` (schema v1.1.0 plano: `create`/`update`/`delete`). Prohibido forjar `{name}.md` a mano sin sello. |
| **L-R11-CRUD-PURE** | `Domain_Entity_Updated` = CRUD genómico. **Prohibido** contaminar con telemetría / `telemetry_snapshot` (Plan B telemetría vigente: `Domain_Entity_Telemetry_Captured`). |
| **L-R11-TRACE** | AC-R11 exige sello **presente y trazable**: evento en bus dominio (`./.events/` vía topología Cúmulo) y/o upsert coherente en `eda-coverage.json` (`last_emitted_event: Domain_Entity_Updated`) por ED mutada; Argos verifica muestra ≥1 mutación R12 + auditoría orphan. |
| **L-R11-NO-BYPASS** | Integridad solo-hash (`hash_signature` + `verify-process-integrity`) **no sustituye** el sello EDA. Ambos pueden coexistir; el gate Done de R11 es el sello, no solo el hash. |
| **L-R12-WAVE** | Ola de migración **controlada** (no barrido caótico del genoma completo). Umbral numérico exacto = handoff **Dedalo** (**Q2**); piso Mayeuta para calificar «masiva» vs piloto H4: **≥8 ED nuevas** homologadas (total **≥16**). Dedalo puede elevar el umbral con justificación de riesgo/valor; no puede bajarlo sin laudo Racso. |
| **L-R12-COHERENCE** | Toda anotación `provides` / `requires_capability` debe ser coherente con `capability-taxonomy.catalog` y, cuando aplique proveedor, con fila en `capability-bindings.md`. Binding table ≠ taxonomía (**L-CODEX-ROLE** reafirmado). |
| **L-R12-NO-INVENT** | Prohibido declarar capacidades fuera del `catalog` vigente. Altas de términos nuevos al Códice = **fuera** de R12 salvo laudo Racso explícito (**Q3**). Por defecto la ola opera sobre `doc:closure` y `proc:git-sync`. |
| **L-R12-MUTATION** | Mutación genoma vía `entity-manager` + registro `SddIA/evolution/` por ED tocada (o lote documentado). Misma aduana DA-4 / Raw Kernel que H4. |
| **L-R12-BLIND-PREF** | Preferir path ciego (`requires_capability` sin `delegates_to` identidad) donde aporte valor medible; no exigir ceguera 100% del catálogo en este ciclo. |
| **L-R13-OPT** | Ampliar piloto EDA DI solo si hay métrica de valor (cobertura de reacción, latencia, o regresión AC-R6 reforzada). **Prohibido** sustituir path sync H2 por EDA-only total sin laudo Racso. |
| **L-RUNTIME-PRESERVE** | Gate, resolver, Cerbero RBAC, envelope y output validator **permanecen**; R11/R12 no reabren diseño de cadena DI salvo bug de regresión. |
| **L-PBI-LOC** | PBI-042 permanece en `docs/todos/pending/` hasta Done global o laudo Racso; este feature no archiva el PBI padre solo por cerrar Hito 5. |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Backfill sello H4 | (A) solo forward: sellar mutaciones R12; (B) backfill explícito de las 8 ED H4 vía `entity-manager` update no-op/metadata + `Domain_Entity_Updated` | AC-R11 exige trazabilidad en mutaciones R12; (B) cierra L-R10-SEAL histórico si la auditoría lo requiere |
| **Q2** | Umbral y lista ola R12 | Dedalo fija `N_ola` (≥8 nuevas) + enumeración ED (process/skill/action) + tipo anotación + fase consumidora | Total homologadas ≥16; sin capacidades inventadas; riesgo acotado por lotes |
| **Q3** | Expansión taxonomía | (A) ninguna alta (default **L-R12-NO-INVENT**); (B) ≤K términos nuevos con laudo Racso + contratos schema + filas binding | Sin (B) no hay proveedores/consumidores fuera de los 2 términos |
| **Q4** | Estrategia de lotes | (A) un PR/lote único; (B) sub-olas en el mismo `persist_ref` con evolution por lote | Control de blast-radius; Argos debe poder auditar conteo acumulado |
| **Q5** | Paths ciegos adicionales | ¿Qué fases nuevas pasan a `requires_capability`-only? | Preferencia **L-R12-BLIND-PREF**; no gate de Done si el umbral de homologación se cumple con mixto |
| **Q6** | R13 — ampliar piloto EDA | (A) omitir en este ciclo; (B) ampliar N consumidores/reactores con métrica | No sustituir sync; regresión AC-R6 verde |
| **Q7** | Evidencia AC-R11 en CI | (A) fixture emit + assert `eda-coverage` / bus; (B) auditoría Argos sobre evolution + coverage post-mutación | Debe ser reproducible sin depender de Shell IDE crudo |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-R11** | Sello `Domain_Entity_Updated` presente y trazable en mutaciones R12 vía `entity-manager` (CRUD puro); aduana EDA coherente (`orphan_count == 0`) | Muestra de mutación(es) R12 con evento/cobertura; sin forja manual huérfana; L-R10-SEAL cerrado en alcance acordado (Q1) |
| **AC-R12** | Ola de migración con umbral Dedalo (piso ≥8 ED nuevas; total ≥16) + coherencia taxonomía + `capability-bindings.md`; mutación vía `entity-manager` + evolution | Diff genoma acotado + evolution; conteo verificable; sin términos fuera de catálogo |

Regresión obligatoria:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-R1** | Resolución ciega sin `delegates_to` hardcodeado | H2 |
| **AC-R2** | `di_binding` en stdin cápsula | H2 |
| **AC-R5** | Cerbero RBAC deny post-gate | H3 |
| **AC-R6** | Piloto EDA async sin bloquear orquestador | H3 |
| **AC-R7** | Término `proc:git-sync` en taxonomía | H3 |
| **AC-R8** | Validación schema payload salida real | H3 |
| **AC-R9** | Cerbero envelope rechaza `di_binding` inválido | H4 |
| **AC-R10** | ≥8 ED homologadas (baseline H4 intacto) | H4 |
| **AC-P1** | Homologación OK → ignición | MVP |
| **AC-P2** | Incumplimiento contrato pre-ignición → DLQ | MVP |
| **AC-P3** | Capacidad no indexada → abort limpio | MVP |

## D5 — Veredicto

**ok** — Requisitos Hito 5 termodinámicamente estables. Handoff a Dedalo: diseño sellado EDA / path `entity-manager`→`Domain_Entity_Updated` (Q1, Q7) + ola migración catálogo (Q2–Q5) + decisión R13 (Q6) → `spec.md` / `plan.md`.
