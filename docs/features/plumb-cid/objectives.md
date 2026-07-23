---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-07-23"
process: feature
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
document_id: LAB-PLUMB-CID
execution_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: mayeuta-stabilization
agents: mayeuta
status: requirements_stable
pbi_status: absent_pending_path
---

# Objetivos — plumb-cid

## Misión

Estabilizar y materializar el lab **plumb-cid**: demostrar trazabilidad auditable del `correlation_id` inyectado por `kalma2-agent-runtime-cursor` a través de la fase Mayeuta del proceso `feature` (`clarify.md` + `objectives.md` bajo `persist_ref` resuelto), sin inventar PBI/TODOs ni éxito git.

## Punto objetivo

> **O-PLUMB-CID:** El `correlation_id` de sesión (`a1b2c3d4-e5f6-4789-a012-3456789abcde`) figura de forma idéntica y machine-readable en el frontmatter de `clarify.md` y `objectives.md` bajo `docs/features/plumb-cid`, con patrón `features-documentation-pattern`; el hueco del PBI referenciado queda explícito; Dedalo recibe este cuerpo como `refined_requirements` para un blueprint lab mínimo de evidencia (sin producto de dominio inventado).

## Alcance

| Dentro | Fuera |
|--------|-------|
| Plumb documental CID (frontmatter clarify/objectives) | Inventar feature de negocio / dominio |
| Resolución `persist_ref` vacío → `docs/features/plumb-cid` | Escribir `docs/todos/` (Mayeuta/Tekton/Argos) |
| Documentar gap PBI ausente | Absorber F3 git-manager KM residual PPR #136 |
| Handoff Dedalo (`refined_requirements`) | Reabrir pasarela Kalma2 / DI / GesFer |
| Intento evidencia vía `skill:git-manager` | Bypass Shell destructivo / inventar stdout |

## Objetivos medibles

| ID | Objetivo | Criterio (AC) |
|----|----------|---------------|
| **O1** | CID en frontmatter | AC-L-CID: mismo `correlation_id` en clarify + objectives |
| **O2** | Cascada Mayeuta | AC-L-DOC: ambos `.md` con frontmatter patrón + cuerpo estabilizado |
| **O3** | Gap PBI | AC-L-PBI: ausencia de `docs/todos/pending/[FEATURE] plumb-cid.md` documentada; no forja KM desde agentes de ejecución |
| **O4** | Evidencia git honesta | AC-L-GIT: stdout `git-manager` o declaración explícita de no materializado |
| **O5** | Cierre lab | AC-DONE-LAB: fases posteriores no inventan APTO sin evidencia física |

## Flujo ontológico objetivo (qué, no cómo)

```text
Runtime (cid inyectado; persist_ref vacío)
  → workspace-init (stub objectives + rama feat/plumb-cid)
  → Mayeuta: clarify.md + objectives.md con cid en frontmatter
  → Dedalo: blueprint lab evidencia CID / gates no-fake
  → Tekton/Argos: materializar solo si runtime permite; sin fake
```

## No objetivos

- Crear el PBI físico desde Mayeuta/Tekton/Argos.
- Ampliar a residuales PPR #136 / PBI-042+ / pasarela async.
- Declarar APTO o evidencia git sin captura física.
- Mutar genoma Core como alcance de este lab.

## Invariantes

- `SddIA/core/cumulo.paths.json` = SSOT de paths (`featurePath` → `docs/features`).
- Git vía `skill:git-manager` (preferente `./sddia-run.sh --tool git-manager`).
- Semillas Kaizen/TODOs solo agent:cumulo o evento `Kaizen_Alert_Required`.
- Bloqueo de runtime ≠ cambio de requisito: ausencia de evidencia = blocked/NO_APTO, no bajar piso.

## Ley aplicada

- `.cursorrules` §4–§5 (cápsulas JSON; agnosticismo Core)
- `features-documentation-pattern` v1.2.1
- Proceso `feature` — fase Estabilización → Dedalo consume este cuerpo como `refined_requirements`
- Clarificaciones D0–D8 y laudos Q1–Q4 en `clarify.md`

## Artefactos de referencia

- Este `persist_ref`: `docs/features/plumb-cid/`
- PBI referenciado (ausente): `docs/todos/pending/[FEATURE] plumb-cid.md`
- Soft-dep operativo (fuera de alcance): `docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md`
- Runtime: `kalma2-agent-runtime-cursor`
- Semilla cruda init: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md»
