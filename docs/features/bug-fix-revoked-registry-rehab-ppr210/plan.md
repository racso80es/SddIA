---
feature_name: bug-fix-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T1-instance-rehab
  - T2-evolution
  - T3-argos
  - T4-doc-archive
  - T5-delivery-close
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr210
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY
uuid: e7a1b2c3-4d5e-6f78-9a0b-1c2d3e4f5a6b
ola: A1
olas:
  - A1
---

# Plan — ola A1 bug-fix-revoked-registry-rehab-ppr210

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T1–T5 en esta sesión.

Init lab: `execution_id` `243b6790-ee2a-42f8-8869-4fbf17a3c16b` · vehículo `feature` · `SDDIA_LAB_SKIP_GIT` (dirty `main`). Checkout formal de rama = T1+ cuando worktree limpio.

## T1 — A1 instancia (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. DELETE `revoked.bug-fix`. Assert `permanent.bug-fix` ausente.
2. Reset absoluto bucket raíz `bug-fix` (**L-RESET-ABS** + **L-SAMPLES** + laudo #210).
3. Assert laterales `revoked.{accept-pr,feature,refactorization}` intactos.
4. Evidencia (campos/timestamp, no secretos) en `execution.md`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`).
2. Entrada `directories.evolution` UUID `e7a1b2c3-4d5e-6f78-9a0b-1c2d3e4f5a6b`.
3. Assert diff: **no** `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales.

## T3 — Argos

`validacion.md`: `global`, checks AC-*, `git_changes`, `pbi_archived: true`, `branch: refactor/bug-fix-revoked-registry-rehab-ppr210`.

## T4 — Archive PBI

Mover canónico `docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210).md` → `docs/todos/done/`.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` (vehículo) / `process_label: refactorization` · `persist_ref` · `branch_name`.

Git: `skill:git-manager`. Post-rehab: smoke `bug-fix` sin re-revocación inmediata.

## Orden

```text
T1 → T2 → T3 → T4 → T5
```

## Delegaciones

| Fase | Cápsula |
|------|---------|
| A1 FS | Tekton `filesystem-ops` |
| Git | `skill:git-manager` |
| PR | `action:execute-process` → `delivery-close-cycle` |

## Fuera de este plan

Rehab laterales; mutación engine; umbrales; ejecución T1–T5 esta sesión.
