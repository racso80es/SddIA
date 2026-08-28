---
feature_name: x
created: "2026-08-28"
process: bug-fix
persist_ref: docs/fixes/x
branch_name: fix/x
correlation_id: cc6d6e2c-b84b-40f9-ac01-acff25ed252e
execution_id: 92716387-568c-42c9-895d-2bf2aa186659
dedalo_verdict: ok
phases:
  - materialize-pbi-cumulo
  - tekton-documental-cascade
  - argos-verification
  - closure-documental
---

# Plan — bug-fix lab smoke (`x`)

## Fases

1. **materialize-pbi-cumulo** — `agent:cumulo` crea `docs/todos/pending/[FIX] x.md` con semilla lab; Tekton/Dedalo no intervienen en KM.
2. **tekton-documental-cascade** — Tekton lee `spec.md` + `plan.md`; actualiza `implementation.md` / `execution.md` bajo `persist_ref`; git vía `skill:git-manager` si RBAC lo permite.
3. **argos-verification** — Argos audita checks CA1–CA7; escribe `validacion.md`; no inventa APTO.
4. **closure-documental** — Pre-merge: mover PBI a `docs/todos/done/`; `pbi_archived: true`; `delivery-close-cycle` con `source_process: bug-fix`.

## Orden de dependencias

```text
cumulo (PBI) → Dedalo (spec/plan) ✓ → Tekton → Argos → cierre documental → delivery-close-cycle
```

## Criterio de parada

Si PBI sigue ausente tras fase 1: Tekton registra `blocked`; no se invoca `delivery-close-cycle`.
