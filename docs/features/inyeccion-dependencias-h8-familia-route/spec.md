---
feature_name: inyeccion-dependencias-h8-familia-route
created: "2026-07-22"
updated: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h8-familia-route
persist_ref: docs/features/inyeccion-dependencias-h8-familia-route
document_id: PBI-043-H8-FAMILIA-ROUTE
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
execution_id: a7c3e91f-2b84-4d6e-9f01-5c8a2e7d4b63
base: main
version_spec: "1.1.0"
agent: dedalo
scope: "Hito 2 (H8) — Familia route residual DI (R4–R5 / AC-H8) · Rama A bus:route"
q1_laudo: alta-bus-route
ac_h8_branch: A
racso_countersign: "2026-07-22T16:56:00Z"
provider: "skill:bus-operator"
---

# Especificación técnica — H8 Familia route (PBI-043 Hito 2 · Rama A)

## 1. Contexto

Laudo Racso **(A)** 2026-07-22: autoriza alta `bus:route` + homologación §3.2. Supersede defer Dedalo previo.

| Vector | Estado post-Rama A |
|--------|-------------------|
| Taxonomía | v1.0.3 (+ `bus:route`) |
| Bindings | v1.2.0 (fila → `skill:bus-operator`) |
| Contrato | `bus.route.schema.json` |
| Provider | `skill:bus-operator` v1.1.0 (`provides` += `bus:route`) |
| §3.2 | 3/3 con `requires_capability` → `bus:route` (mixto `agent:cumulo`) |
| Inventario | **29** with / **13** without (post-ola) |

## 2. Alcance

| ID | Entregable |
|----|------------|
| **R4** | Alta `bus:route` (taxonomía + schema + binding + provides) |
| **R5** | Homologar `route-domain`, `route-orchestration`, `route-telemetry` · `N_ola=3` |
| **Q8** | Revalidar `route-domain-event` (`fs:persist` ×3) — noop |
| **Sellos** | `Domain_Entity_Updated` + evolution + orphan 0 |
| **Regresión** | `capability_di` / `cerbero_di` |

**Fuera:** H9–H10; R10; archivo PBI-043; runtime DI rewrite.

## 3. Laudos

| ID | Laudo |
|----|-------|
| **Q1** | **(A)** Alta `bus:route` — countersign Racso |
| **Q2** | Mixto: `requires_capability` + conservar `delegates_to: agent:cumulo` |
| **Q3** | Provider canónico = `skill:bus-operator` |
| **Q4** | Un PR / un lote |
| **Q5** | Sellos + orphan scan + suites DI |
| **Q6** | Pack MVP→H7 `capability_di` + `cerbero_di` |
| **Q7** | `provides` en bus-operator (obligatorio para gate/resolver) |
| **Q8** | RDE sin drift → noop |

## 4. Forma canónica fase §3.2

```yaml
requires_capability:
  - id: "bus:route"
    contract: "bus.route"
    version: ">=1.0.0"
delegates_to:
  - agent:cumulo
```

Runtime residual fractal bypassa fases DI (handler nativo); anotación genómica + gate/resolver coherentes cuando el path genérico se active.

## 5. Criterios AC-H8 Rama A

| ID | Criterio |
|----|----------|
| **AC-H8** | 3/3 §3.2 DI `bus:route`; taxonomía/bindings/schema/provides alineados |
| **AC-NO-INVENT** | Alta solo tras laudo Racso (A) |
| **AC-SEAL** | Sellos EDA en mutaciones ED |
| **AC-ORPHAN** | `orphan_count == 0` |
| **AC-REG-DI** | Suites verdes |
| **AC-INV** | Recuento post-ola documentado |

PBI-043 permanece en `pending/` (`pbi_archived: false`).
