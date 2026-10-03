---
feature_name: plumb-cid
created: "2026-07-23"
updated: "2026-10-03"
process: feature
branch_name: feat/plumb-cid
persist_ref: docs/features/plumb-cid
pbi_ref: docs/todos/pending/[FEATURE] plumb-cid.md
document_id: LAB-PLUMB-CID
correlation_id: a1b2c3d4-e5f6-4789-a012-3456789abcde
phase: mayeuta-stabilization
agents: mayeuta
status: requirements_stable
pbi_status: absent_pending_path
git_evidence_mayeuta: not_materialized_shell_rejected
execution_id: "ebcaeecd-964b-4ffa-81bc-74ded9a512a7"
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
| `persist_ref` = `docs/features/plumb-cid` (`featurePath`) | Escribir `docs/todos/` (Mayeuta/Tekton/Argos) |
| Documentar gap PBI ausente (reconfirmado 2026-10-03 / exec ebcaeecd) | Absorber deudas Tracker / F3 git-manager residual |
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
Runtime (cid inyectado)
  → Mayeuta: clarify.md + objectives.md con cid en frontmatter
  → Dedalo: blueprint lab evidencia CID / gates no-fake
  → Tekton/Argos: materializar solo si runtime permite; sin fake
```

## Estado de estabilización (2026-10-03 / execution ebcaeecd-…)

| Vector | Estado |
|--------|--------|
| L-CID-FM / O1 | Cumplido — CID idéntico en FM de ambos artefactos |
| L-PERSIST / O2 | Cumplido — patrón documental bajo `persist_ref` |
| L-PBI-GAP / O3 | Documentado — PBI físico **ausente** (Glob 0 hits `*plumb*` en `docs/todos/`) |
| L-GIT / O4 | **No materializado** — `./sddia-run.sh --tool git-manager` → Rejected (sin stdout) |
| L-NO-FAKE / O5 | Vigente — ausencia ≠ éxito |

## No objetivos

- Crear el PBI físico desde Mayeuta/Tekton/Argos.
- Ampliar a residuales Tracker / PBI-042+ / pasarela async.
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
- Clarificaciones D0–D9 y laudos Q1–Q4 en `clarify.md`

## Artefactos de referencia

- Este `persist_ref`: `docs/features/plumb-cid/`
- PBI referenciado (ausente): `docs/todos/pending/[FEATURE] plumb-cid.md`
- Runtime: `kalma2-agent-runtime-cursor`
- Semilla cruda init: «inicia feature docs/todos/pending/[FEATURE] plumb-cid.md»
- `execution_id`: `ebcaeecd-964b-4ffa-81bc-74ded9a512a7`
