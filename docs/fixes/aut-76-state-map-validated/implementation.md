---
feature_name: aut-76-state-map-validated
created: "2026-11-06"
process: bug-fix
phase: implementation
agents: tekton
branch_name: fix/aut-76-state-map-validated
persist_ref: docs/fixes/aut-76-state-map-validated
pbi_ref: AUT-76
document_id: PBI-222b4def
uuid: 222b4def-325a-4645-80d3-fbb15bf7d34b
execution_id: "6c28414f-8a36-4f66-b9e3-4969dc642978"
---

# Implementación — AUT-76

## T1 — Aduana

`TRACKER_STATE_KEYS` en `project_binding.rs` incluye `validated` entre `in_validation` e `in_review`.

Test `contract_accepts_state_map_validated_and_rejects_unknown_key`: manifiesto Core-like con `validated: Validated` pasa `bind`; clave `bogus` sigue `PROJECT_CONFIG_INVALID`.

## T2 — Anti-retroceso

`state_rank`: `validated` 4, `in_review` 5, `done` 6. Tests `state_rank_orders_cycle` y `discard_when_issue_already_at_or_past_target`.

## T3 — Auditoría

`docs/audits/ciclo-vida-software-sddia.md` §3.4: fila `validated` rango 4; `in_review` 5; `done` 6. Anti-retroceso: `validated` descarta `in_validation` rezagado.

## T4 — Contrato

`project-config-contract` **1.4.0 → 1.5.0**. Conjunto `{backlog, todo, in_progress, in_validation, validated, in_review, done, cancelled}`.

`entity-manager` no tiene clase `contract` (el artefacto vive bajo `library/codexes/.../contracts/`). Precedente HU-A 01: edición del fichero + evolution. DA-2 aplica cuando hay creator.

## T5 — Evolution

`6f997575-80ab-431f-ae2a-9fa296eb69f6` (`EVOL_OK`, alta). Hash `sha256:0d60cee2128ef008d1ec5681c96d360710a1c7f882a507399370866bfe450176`.
