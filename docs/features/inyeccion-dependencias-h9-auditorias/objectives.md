---
feature_name: inyeccion-dependencias-h9-auditorias
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h9-auditorias
persist_ref: docs/features/inyeccion-dependencias-h9-auditorias
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H9-AUDITORIAS
execution_id: c9e4b17a-6f2d-4a8e-9c3b-1d5e8f0a7b42
phase: tekton-done
agents: mayeuta,dedalo,tekton,argos
r6_status: laudoed-rama-a
r6_dedalo_provisional: R6-A-qa-probe-tool-provider
ac_h9_branch: A
racso_countersign: "2026-07-23T06:53:00Z"
---

# Objetivos — inyeccion-dependencias-h9-auditorias

## Misión

Materializar el **Hito 3 (H9)** del residual PBI-043 (post H8 PR #147 merge `85052a8`): homologar las **auditorías §3.3** (`N_ola=5`) con DI por capacidades bajo modelo **R6**, tras laudo Racso (alta `qa:probe` + tool-provider **o** defer documentado).

## Estado de partida (innegociable)

| Vector | Estado |
|--------|--------|
| H7 núcleo FS | **Hecho (main)** — PR #144 |
| H8 familia route | **Hecho (main)** — PR #147 merge `85052a8`; alta `bus:route` |
| Runtime DI (gate / resolver / Cerbero RBAC+envelope / output validator) | **Entregado** — providers solo `skill:`\|`action:`; `tool:` excluido |
| Taxonomía | `doc:closure`, `proc:git-sync`, `fs:persist`, `bus:route` (v1.0.3) |
| Bindings | `capability-bindings.md` v1.2.0 |
| Process con `requires_capability` | **29** |
| Process sin `requires_capability` (AC-INV 2026-07-22 post-H8) | **13** — drift 0 vs PBI §3.3+§3.4 |
| PBI-043 | Abierto en `docs/todos/pending/` (`pbi_archived: false`) |

## Vectores soberanos (este ciclo = R6–R7 / AC-H9)

1. **R6 — Modelo DI tool-bound (PBI Q2):** candidata provisional `qa:probe` (`qa.probe`) + extensión mínima runtime para provider `tool:` + `provides` en tools chaos/audit + binding canónico. Alta Códice/runtime **solo** con laudo Racso (**L-R6-GATE** / **L-NO-INVENT** / **L-TEKTON-GATE**). Si laudo = defer o sin laudo: documentar defer y **no** mutar genoma/runtime (**L-DEFER-OK**). Reuso de capacidad vigente solo con prueba semántica — Dedalo provisional: **improbable** (**L-REUSE-GATE**).
2. **R7 — Homologar exactamente 5 ED §3.3** bajo el modelo R6:
   `audit-sandbox-isolation-rbac`, `audit-telemetry-compliance-breach`, `audit-thermodynamic-toll-failsoft`, `event-bus-audit`, `telemetry-compliance-audit`.
   `N_ola ≥ 5` (piso = las 5; no bajar sin laudo Racso).
3. **Mutación** vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0` (**AC-SEAL** / **AC-ORPHAN**).
4. **Regresión** suites DI MVP→H8 (`capability_di` / `cerbero_di`) (**AC-REG-DI**).
5. **L-PBI-LOC** — PBI-043 permanece en `pending/`; H10 fuera.

## Criterios de aceptación — producto Hito 3

| ID | Criterio |
|----|----------|
| **AC-H9** | **Rama A:** 5/5 §3.3 con DI coherente taxonomía+bindings bajo R6; sellos EDA; orphan 0; runtime preservado o patch mínimo laudoado. **Rama B:** defer explícito documentado; sin altas inventadas al Códice ni patch runtime no laudoado |
| **AC-INV** | Inventario recontado; drift documentado en `clarify.md` D1 |
| **AC-NO-INVENT** | Sin altas libres al Códice / bindings / runtime sin laudo Racso |
| **AC-SEAL** | Mutación vía entity-manager + `Domain_Entity_Updated` + evolution (si hay mutación ED) |
| **AC-ORPHAN** | `orphan_count == 0` |
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes (MVP→H8) |

## Restricciones y leyes aplicadas

- Gate/resolver/Cerbero RBAC/envelope/output validator se **conservan**; extensión `tool:` solo si R6-A laudoado (**L-RUNTIME-PRESERVE** / **L-RUNTIME-MIN**).
- Capacidades solo del catálogo vigente salvo laudo Racso para alta (**L-R6-GATE** / **L-NO-INVENT**).
- Mutación genoma vía `entity-manager` + sello + evolution (**L-R2-MUTATION**). Git solo vía `skill:git-manager`.
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- PBI-043 no se archiva en este ciclo (**L-PBI-LOC**).
- **Tekton bloqueado en R6-A** hasta countersign Racso (**L-TEKTON-GATE**).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| H10 gobernanza/interactores (§3.4) | Ciclo posterior |
| R10 EDA-only total | Solo laudo Racso |
| Reescritura amplia runtime DI | Fuera; solo patch mínimo R6-A si laudo |
| GesFer / Paciente 0 / Fractura Core F1 | Otro PBI / `persist_ref` |
| Altas libres al Códice | Fuera salvo laudo Racso |
| Deuda PPR #136 delivery-close revoked | PBI distinto |
| Archivo PBI-043 padre | Solo Done global H7–H10 |

## Dictamen Dedalo provisional (R6)

| Rama | Estado |
|------|--------|
| **R6-A** `qa:probe` + tool-provider | **Preferida** — pendiente laudo Racso |
| **R6-B** defer | Default operativo sin laudo |
| **R6-C** reuso catálogo | **Descartada** (semántica incoherente) |

Ambigüedades post-laudo: ver `clarify.md` §D3 Q2–Q8 (provider canónico, densidad DI, `provides` tools, lotes, evidencia, smoke, superficie patch).

## Handoff

Mayeuta+Dedalo **DONE** (requisitos estables; R6 enmarcado). Siguiente:

1. **Racso** — countersign R6-A (`qa:probe` u id equivalente) **o** mandato R6-B defer.
2. **Dedalo** → `spec.md` / `plan.md` solo tras (1).
3. **Tekton** — materializa según rama laudoada; **prohibido** inventar capacidad sin countersign.
