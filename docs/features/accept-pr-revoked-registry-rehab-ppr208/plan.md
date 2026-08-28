---
feature_name: accept-pr-revoked-registry-rehab-ppr208
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
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
ola: A1
olas:
  - A1
---

# Plan — ola A1 accept-pr-revoked-registry-rehab-ppr208

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T1–T5 en esta sesión.

Init lab: `execution_id` `e1de4691-5b6f-495b-85ff-b6a52dcd11c4` · vehículo `feature` · `SDDIA_LAB_SKIP_GIT` (dirty `main`). Checkout formal de rama = T1+ cuando worktree limpio.

## T1 — A1 instancia (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. DELETE `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Reset absoluto bucket raíz `accept-pr` (**L-RESET-ABS** + **L-SAMPLES** + laudo #210).
3. Assert laterales `revoked.{accept-pr,feature,refactorization}` intactos.
4. Evidencia (campos/timestamp, no secretos) en `execution.md`.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`).
2. Entrada `directories.evolution` UUID `d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20`.
3. Assert diff: **no** `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales.

## T3 — Argos

`validacion.md`: `global`, checks AC-*, `git_changes`, `pbi_archived: true`, `branch: refactor/accept-pr-revoked-registry-rehab-ppr208`.

## T4 — Archive PBI

Mover canónico `docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #210).md` → `docs/todos/done/`.

## T5 — DCC

`delivery-close-cycle` · `source_process: feature` (vehículo) / `process_label: refactorization` · `persist_ref` · `branch_name`.

Git: `skill:git-manager`. Post-rehab: smoke `accept-pr` sin re-revocación inmediata.

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
