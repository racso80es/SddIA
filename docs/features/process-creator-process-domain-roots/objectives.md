---
feature_name: process-creator-process-domain-roots
created: "2026-08-10"
process: feature
branch_name: feat/process-creator-process-domain-roots
persist_ref: docs/features/process-creator-process-domain-roots
pbi_ref: docs/todos/pending/[ARQUITECTURA] process-creator — jurisdicción process_domain_roots (ABSTRACT-03 D7).md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR
phase: mayeuta-stabilization
agents: mayeuta
source_feature: docs/features/sddia-domain-abstract-03-relocalizacion
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
correlation_id: ""
---

# Objetivos — process-creator-process-domain-roots

## Misión

Liquidar la deuda **D7** diferida en ABSTRACT-03 (`PBI-SDDIA-DOMAIN-ABSTRACT-03-D7-PROCESS-CREATOR`, uuid `a3c7e91f-2b4d-4f8a-9c1e-7d6b0a5f3211`): tras el packing de los 6 process software-lifecycle al códice y la introducción de `process_domain_roots` (Cúmulo **1.6.0**, laudo `L-PACK-MULTIROOT-SIX-MOVE`), hacer que **process-creator** (o forja sustituta gobernada) resuelva el **destino de persistencia e índice** vía topología Cúmulo + política de jurisdicción — software-lifecycle / packing códice → `process_domain_roots`; resto → `directories.process` (Core) — sin filas fantasma en el índice Core ni artefactos ejecutables de dominio bajo `SddIA/process/`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Política de jurisdicción de escritura en `process-creator` (y touchpoints de forja asociados) | Re-mover / re-forjar los 6 ya packing |
| Persistencia + índice en root destino; unicidad cross-root | Migrar `entity-manager`, daemons, routes EDA (`L-KEEP-CORE`) |
| Smoke/AC: alta software no deja ejecutable bajo Core `SddIA/process/` | Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136) |
| Documentar overlay instancia (`.SddIA/local.paths.json`) si aplica, o N/A | Reabrir AC-MOVE / resolve de ABSTRACT-03 salvo regresión demostrada |
| Cascada documental + cierre single-PR | Hardcode de paths de cliente fuera de Cúmulo ± overlay |

## Hitos

1. **H1 — Diseño jurisdicción:** classifier + path destino (Dedalo `spec`/`plan`).
2. **H2 — Forja multi-root:** persistencia + índice destino + anti-fantasma Core.
3. **H3 — Verificación:** AC-SMOKE / AC-RESOLVE-COMPAT / AC-DOC.

## Criterios de aceptación

- **AC-JURIS / AC-INDEX / AC-SMOKE / AC-UNIQ / AC-RESOLVE-COMPAT / AC-OVERLAY / AC-BUILD / AC-DOC / AC-NONSCOPE** (ver `clarify.md` D4).

## Ley aplicada

- Rutas solo vía `SddIA/core/cumulo.paths.json` (y fusión local documentada).
- Git exclusivamente vía `skill:git-manager` / `./sddia-run.sh --tool git-manager` (sin bypass raw destructivo).
- Genoma vía forja gobernada / `entity-manager`; `process-creator` permanece Core (escritura multi-root).
- `features-documentation-pattern` v1.2.1; cierre documental en rama (un PR).
- Jerarquía: Acción → Agente → Skill → Tools.
- Filtro C: no inventar éxito; no re-move de packing.

## Handoff Dedalo

Consumir este cuerpo + `clarify.md` como `refined_requirements`. Diseñar `spec.md` + `plan.md` bajo **L-JURIS** / **L-INDEX-TARGET** / **L-UNIQ-MULTI**; fijar classifier y touchpoints reales de escritura antes de autorizar a Tekton.
