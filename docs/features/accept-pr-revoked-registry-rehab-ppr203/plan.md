---
feature_name: accept-pr-revoked-registry-rehab-ppr203
created: "2026-08-27"
process: refactorization
phase: planning
agents: dedalo
phases:
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
ola: A1
olas:
  - A1
---

# Plan — ola A1 accept-pr-revoked-registry-rehab-ppr203

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** no ejecutar T1/T2 en esta sesión.

## T1 — A1 instancia (AC-A1 / AC-ONTO / AC-GIT-CLEAN)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`. **Fuera del diff git.**

1. DELETE `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Reset absoluto bucket raíz `accept-pr` (**L-RESET-ABS** + **L-SAMPLES** + laudo #203).
3. Assert `revoked.refactorization` intacto.
4. Evidencia (campos/timestamp, no secretos) en `execution.md`.

**Orden host:** completar ola A2 (motor en `main` local / tests verde) **antes** de T1.

## T2 — Documental + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón; `items` / `items_applied`).
2. Entrada `directories.evolution` UUID `b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e` (compartido con A2; una fila, dos olas).
3. Assert diff: **no** `.SddIA/cerbero/` ni `.SddIA/radamanto/` ni umbrales.

## Fuera de este plan

T3–T5 Argos / archive PBI / DCC: **plan A2** (cierre único de PBI). Mutación engine. Rehab laterales.

## Delegaciones

| Fase | Cápsula |
|------|---------|
| A1 FS | Tekton `filesystem-ops` |
| Git | `skill:git-manager` |
