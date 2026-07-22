---
feature_name: inyeccion-dependencias-h7-nucleo-fs
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h7-nucleo-fs
persist_ref: docs/features/inyeccion-dependencias-h7-nucleo-fs
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H7-NUCLEO-FS
execution_id: b8e2a471-5c9d-4f3a-8e1b-6d0c9f2a4b7e
phase: mayeuta-stabilization
agents: mayeuta
---

# Objetivos — inyeccion-dependencias-h7-nucleo-fs

## Misión

Materializar el **Hito 1 (H7)** del residual PBI-043 (post Done PBI-042 PR #142 merge `90424f4`): homologar el **núcleo FS** (§3.1) con DI por capacidades (`requires_capability` → `fs:persist`, path ciego preferente), preservando taxonomía+bindings, sellos EDA y regresiones MVP→H6.

## Estado de partida (innegociable)

| Vector | Estado |
|--------|--------|
| Runtime DI (gate / resolver / Cerbero RBAC+envelope / output validator) | **Entregado** — no reescribir salvo bug |
| Taxonomía | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings | `capability-bindings.md` v1.1.0 |
| Process con `requires_capability` | **18** |
| Process sin `requires_capability` (AC-INV 2026-07-22) | **24** — drift 0 vs PBI |
| PBI-043 | Abierto en `docs/todos/pending/` |

## Vectores soberanos (este ciclo = R1–R3 / AC-H7)

1. **R1 — Homologar exactamente 8 ED §3.1** con `requires_capability` → `fs:persist` (path ciego preferente):
   `entity-manager`, `route-domain-event`, `daemon-kill-switch`, `governance-daemon-manager`,
   `daemon-heartbeat-audit`, `fix-tool-process`, `telemetry-batch-stub`, `workspace-smoke`.
   `N_ola ≥ 8` (piso = las 8; no bajar sin laudo Racso).
2. **R2 — Mutación** vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0`.
3. **R3 — Regresión** suites DI MVP→H6 (`capability_di` / `cerbero_di`).
4. **AC-INV / AC-NO-INVENT** — inventario recontado al start; prohibido inventar `capability_id`. Q3 `entity-manager`: preferir ciego `fs:persist` en fases FS; mixto solo si forge lo exige.
5. **L-PBI-LOC** — PBI-043 permanece en `pending/` (`pbi_archived: false`); H8–H10 fuera.

## Criterios de aceptación — producto Hito 1

| ID | Criterio |
|----|----------|
| **AC-H7** | 8/8 §3.1 con DI coherente taxonomía+bindings; sellos EDA; orphan 0; runtime preservado |
| **AC-INV** | Inventario recontado; drift documentado en `clarify.md` D1 |
| **AC-NO-INVENT** | Sin altas libres al Códice / bindings |
| **AC-SEAL** | Mutación vía entity-manager + `Domain_Entity_Updated` + evolution |
| **AC-ORPHAN** | `orphan_count == 0` |
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes |

## Restricciones y leyes aplicadas

- Gate/resolver/Cerbero RBAC/envelope/output validator se **conservan** (**L-RUNTIME-PRESERVE**).
- Capacidades solo del catálogo vigente; default sin altas (**L-NO-INVENT**).
- Preferencia path ciego en fases solo-FS; mixto permitido donde coexisten shell/bus/crypto/execute-process (**L-BLIND-PREF** / **L-Q3-EM**).
- Mutación genoma vía `entity-manager` + sello + evolution (**L-R2-MUTATION**). Git solo vía `skill:git-manager`.
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- PBI-043 no se archiva en este ciclo (**L-PBI-LOC**).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| H8 familia route | Ciclo posterior |
| H9 auditorías | Ciclo posterior |
| H10 gobernanza/interactores | Ciclo posterior |
| R10 EDA-only total | Solo laudo Racso |
| GesFer / Paciente 0 / Fractura Core F1 | Otro PBI / `persist_ref` |
| Altas libres al Códice | Fuera salvo laudo Racso |
| Deuda PPR #136 delivery-close revoked | PBI distinto |
| Archivo PBI-043 padre | Solo Done global H7–H10 |

## Ambigüedades Dedalo — abiertas

Ver `clarify.md` §D3: **Q1** densidad ciega · **Q2** lotes · **Q3** evidencia sello/orphan · **Q4** smoke regresión · **Q5** mixto daemon-* · **Q6** `provides`.

## Handoff

Mayeuta **DONE** (requisitos estables). Siguiente: **Dedalo** → `spec.md` / `plan.md` consumiendo este cuerpo como `refined_requirements` (ola H7, `N_ola=8`, path ciego `fs:persist`, sellos EDA).
