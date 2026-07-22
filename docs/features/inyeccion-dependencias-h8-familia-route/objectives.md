---
feature_name: inyeccion-dependencias-h8-familia-route
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h8-familia-route
persist_ref: docs/features/inyeccion-dependencias-h8-familia-route
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H8-FAMILIA-ROUTE
execution_id: a7c3e91f-2b84-4d6e-9f01-5c8a2e7d4b63
phase: tekton-rama-a-done
agents: mayeuta,dedalo,tekton,argos
q1_laudo: alta-bus-route
ac_h8_branch: A
racso_countersign: "2026-07-22T16:56:00Z"
---

# Objetivos — inyeccion-dependencias-h8-familia-route

## Misión

Materializar el **Hito 2 (H8)** del residual PBI-043 (post H7 PR #144 merge `8f882b8`): homologar la **familia route** (§3.2) con DI `bus:route` tras **laudo Racso (A)** — alta Códice + binding + provider `skill:bus-operator` + ola `N_ola=3`.

## Estado de partida (innegociable)

| Vector | Estado |
|--------|--------|
| H7 núcleo FS | **Hecho (main)** — PR #144 merge `8f882b8` |
| Runtime DI (gate / resolver / Cerbero RBAC+envelope / output validator) | **Entregado** — no reescribir salvo bug |
| Taxonomía | `doc:closure`, `proc:git-sync`, `fs:persist` (v1.0.2) |
| Bindings | `capability-bindings.md` v1.1.0 |
| Process con `requires_capability` | **26** |
| Process sin `requires_capability` (AC-INV 2026-07-22 post-H7) | **16** — drift 0 vs PBI §3.2+§3.3+§3.4 |
| PBI-043 | Abierto en `docs/todos/pending/` (`pbi_archived: false`) |

## Vectores soberanos (este ciclo = R4–R5 / AC-H8)

1. **R4 — Decidir capacidad route (PBI Q1):** candidata provisional `bus:route` u equivalente. Alta taxonomía+binding+contrato **solo** con laudo Racso explícito. Prohibido inventar `capability_id` (**AC-NO-INVENT** / **L-NO-INVENT**). Si laudo = defer: documentar defer y **no** mutar genoma de capacidades (**L-DEFER-OK**). Reuso de capacidad vigente solo con prueba semántica Dedalo (**L-REUSE-GATE**).
2. **R5 — Homologar exactamente 3 ED §3.2** bajo el modelo R4:
   `route-domain`, `route-orchestration`, `route-telemetry`.
   Revalidar `route-domain-event` (`fs:persist` H7) si hay drift (**L-RDE-REVAL**).
   `N_ola ≥ 3` (piso = las 3; no bajar sin laudo Racso).
3. **Mutación** vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0` (**AC-SEAL** / **AC-ORPHAN**).
4. **Regresión** suites DI MVP→H7 (`capability_di` / `cerbero_di`) (**AC-REG-DI**).
5. **L-PBI-LOC** — PBI-043 permanece en `pending/`; H9–H10 fuera.

## Criterios de aceptación — producto Hito 2

| ID | Criterio |
|----|----------|
| **AC-H8** | **Rama A:** 3/3 §3.2 con DI coherente taxonomía+bindings; sellos EDA; orphan 0; runtime preservado. **Rama B:** laudo explícito de defer documentado; sin altas inventadas al Códice |
| **AC-INV** | Inventario recontado; drift documentado en `clarify.md` D1 |
| **AC-NO-INVENT** | Sin altas libres al Códice / bindings sin laudo Racso |
| **AC-SEAL** | Mutación vía entity-manager + `Domain_Entity_Updated` + evolution (si hay mutación ED) |
| **AC-ORPHAN** | `orphan_count == 0` |
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes (MVP→H7) |

## Restricciones y leyes aplicadas

- Gate/resolver/Cerbero RBAC/envelope/output validator se **conservan** (**L-RUNTIME-PRESERVE**).
- Capacidades solo del catálogo vigente salvo laudo Racso para alta (**L-R4-Q1-GATE** / **L-NO-INVENT**).
- Mutación genoma vía `entity-manager` + sello + evolution (**L-R2-MUTATION**). Git solo vía `skill:git-manager`.
- Normas: `features-documentation-pattern`, `capability-taxonomy`, `external-ai-constraints`, `capsule-json-io`, `CONSTITUTION_CORE` Filtro A.
- PBI-043 no se archiva en este ciclo (**L-PBI-LOC**).

## Fuera de alcance (explícito)

| Ítem | Destino |
|------|---------|
| H9 auditorías (§3.3) | Ciclo posterior |
| H10 gobernanza/interactores (§3.4) | Ciclo posterior |
| R10 EDA-only total | Solo laudo Racso |
| Reescritura runtime DI | Fuera salvo bug |
| GesFer / Paciente 0 / Fractura Core F1 | Otro PBI / `persist_ref` |
| Altas libres al Códice | Fuera salvo laudo Racso |
| Deuda PPR #136 delivery-close revoked | PBI distinto |
| Archivo PBI-043 padre | Solo Done global H7–H10 |

## Ambigüedades Dedalo — abiertas

Ver `clarify.md` §D3: **Q1** capacidad (`bus:route` alta vs defer vs reuso · laudo Racso) · **Q2** densidad DI/`delegates_to` · **Q3** provider binding · **Q4** lotes · **Q5** evidencia · **Q6** smoke · **Q7** `provides` · **Q8** drift `route-domain-event`.

## Handoff

Mayeuta **DONE** (requisitos estables). Siguiente: **Dedalo** → `spec.md` / `plan.md` consumiendo este cuerpo como `refined_requirements` (resolver Q1 antes de Tekton; `N_ola=3`; prohibido inventar capacidad sin laudo).
