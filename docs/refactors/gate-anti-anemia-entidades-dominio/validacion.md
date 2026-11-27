---
feature_name: gate-anti-anemia-entidades-dominio
created: "2026-11-27"
updated: "2026-11-27T07:30:00Z"
process: pull-request-review
phase: Veredicto y bloqueo
agent: argos
branch: refactor/gate-anti-anemia-entidades-dominio
branch_name: refactor/gate-anti-anemia-entidades-dominio
persist_ref: docs/refactors/gate-anti-anemia-entidades-dominio
tracker_provider: linear
tracker_ref: AUT-125
pr_url: https://github.com/racso80es/SddIA/pull/480
audit_event_reference: "3899742f-04f5-480b-a1f6-866d93d1d5a3"
audit_report_ref: docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md
pr_audited_event_id: "e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677"
superseded_verdict: "PPR CID 415375d4-a786-49b1-ae94-e883ab4b097e (PASS_F5_VERDICT, fase interna) invalidado por REJECT"
ci_run_id_superseded: "45950436287"
base_commit_sha: fd7ef31b51e795d1aaea2aba53b7cae693a019db
head_commit_sha_fs: 9d7b71725ff93ed40600f7623f48489b09b88e46
source_process: refactorization
global: NO_APTO
verdict: rechazado
approval_status: rechazado
delivery_state: failed
resolution: REJECT
pbi_archived: false
accept_pr_handoff: false
accept_pr_handoff_status: blocked
accept_pr_block_reason: "Auditoría técnica estricta NO_APTO (REJECT): 7 hallazgos bloqueantes registrados en docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md"
checks:
  AC-1: NO_APTO
  AC-2: NO_APTO
  AC-3: NO_APTO
  AC-4: NO_APTO
  F2_DOC_GATE: NO_APTO
  F3_TECH_GATE: NO_APTO
  F4_RBAC_GATE: APTO
  F5_VERDICT_GATE: NO_APTO
  INDEX_SYMMETRY: NO_APTO
  ACTIONS_CONTRACT_INTEGRITY: NO_APTO
  FORGES_INTEGRITY: NO_APTO
  PR_CI_EVIDENCE: NO_APTO
blocking_findings:
  - "actions-contract.md#1,3: pérdida de inputs/outputs, mutación de context y borrado de especificaciones/seguridad en 7 acciones (enrich-fracture-pbi-kaizen, sync-entity-index, purge-sandbox-cache, emit-tracker-sync-failed, emit-validation-started-event, emit-work-initiated-event, purge-repo-retention)"
  - "index-symmetry: corrupción de SddIA/actions/index.md (capabilities vacías y 237 líneas duplicadas), conteo domain=0 en SddIA/events/index.md y 25 UUIDs rotados sin sincronizar agents/process/skills/suites/events/domain index.md ni purgar SddIA/core/eda-coverage.json"
  - "semver-integrity: downgrade de versiones en codex-kalma2-assistant.md, codex-software-engineering.md y features-documentation-pattern.md"
  - "AUT-125/Q1,KA-DOC-1,KA-DOC-2: remediación boilerplate en 71 entidades, audit-inventory.json sin inventario previo y bypass de reglas contractuales en entity_no_anemia.rs"
  - "forges-integrity: pérdida de metadatos en patch_action_content_update, borrado de cola en sync_action_index_row, ausencia de rollback en run_action_forge/run_agent_forge y omisión silenciosa en run_suite_forge"
  - "pr-hygiene: árbol de trabajo sucio con drift en .SddIA/observability/map-snapshot.json y ausencia de evidencia CI verde"
---

# Validación — Auditoría Técnica Estricta (`AUT-125`)

## Veredicto

**NO_APTO** — `resolution: REJECT` · `verdict: rechazado` · `delivery_state: failed` · `accept_pr_handoff: false` (`blocked`).

Informe completo con trazabilidad `ruta:línea`, hallazgos Kaizen, huecos de prueba y payload ECST `PullRequest_Audited`:
- [`docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md`](../../audits/pr-audit-aut-125-gate-anti-anemia-20261127.md)

## Resumen de bloqueantes

| Gate / Criterio | Estado | Motivo principal |
|---|---|---|
| `ACTIONS_CONTRACT_INTEGRITY` | **NO_APTO** | Borrado de especificaciones, `inputs`/`outputs`, `context` y gate regex en 7 acciones (`enrich-fracture-pbi-kaizen`, `sync-entity-index`, `purge-sandbox-cache`, `emit-tracker-sync-failed`, `emit-validation-started-event`, `emit-work-initiated-event`, `purge-repo-retention`). |
| `INDEX_SYMMETRY` | **NO_APTO** | `SddIA/actions/index.md` corrupto (`Capabilities` vacías y 237 líneas duplicadas), `SddIA/events/index.md` con `domain = 0` y 25 UUIDs rotados sin sincronizar índices ni `eda-coverage.json`. |
| `F3_TECH_GATE` / `AC-1..AC-4` | **NO_APTO** | Bypass de `rule.second_kw`/`purpose_kw`, falsos positivos en `truncated_tail`, bugs en `scan_changed` (`base_ref HEAD`, prefijo sin `/`, sin `rank`), y bugs destructivos en `forges/common.rs` y `forges/factory.rs`. |
| `F2_DOC_GATE` | **NO_APTO** | Remediación boilerplate en 71 entidades, downgrades SemVer en 2 códices y 1 norma, `audit-inventory.json` sin inventario pre-remediación (`KA-DOC-1`). |
| `PR_CI_EVIDENCE` | **NO_APTO** | Auditoría estática sin evidencia de CI verde en `.github/workflows/sddia-index-qa.yml` y worktree sucio. |

## Payload ECST (`PullRequest_Audited`)

Emitido vía `./sddia-run.sh --action emit-pr-audited-event` (`emitter_agent: argos`): `event_id` `e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677`, `eda://domain/e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677`. El run CI `45950436287` (verde @ `9d7b7172`) no acredita conformidad: el gate global se satisfizo con boilerplate.

```json
{
  "audit_event_reference": "3899742f-04f5-480b-a1f6-866d93d1d5a3",
  "target_entity_id": "refactor/gate-anti-anemia-entidades-dominio",
  "resolution": "REJECT",
  "tracker_ref": "AUT-125",
  "violated_rules": [
    "actions-contract.md#1,3: pérdida de inputs/outputs, mutación de context y borrado de especificaciones/seguridad en 7 acciones (enrich-fracture-pbi-kaizen, sync-entity-index, purge-sandbox-cache, emit-tracker-sync-failed, emit-validation-started-event, emit-work-initiated-event, purge-repo-retention)",
    "index-symmetry: corrupción de SddIA/actions/index.md (capabilities vacías y 237 líneas duplicadas), conteo domain=0 en SddIA/events/index.md y 25 UUIDs rotados sin sincronizar agents/process/skills/suites/events/domain index.md ni purgar SddIA/core/eda-coverage.json",
    "semver-integrity: downgrade de versiones en codex-kalma2-assistant.md, codex-software-engineering.md y features-documentation-pattern.md",
    "AUT-125/Q1,KA-DOC-1,KA-DOC-2: remediación boilerplate en 71 entidades, audit-inventory.json sin inventario previo y bypass de reglas contractuales en entity_no_anemia.rs",
    "forges-integrity: pérdida de metadatos en patch_action_content_update, borrado de cola en sync_action_index_row, ausencia de rollback en run_action_forge/run_agent_forge y omisión silenciosa en run_suite_forge",
    "pr-hygiene: árbol de trabajo sucio con drift en .SddIA/observability/map-snapshot.json y ausencia de evidencia CI verde"
  ]
}
```

## Evidencia local post-remediación (Tekton, no veredicto)

HEAD `e040895a`. Este bloque no altera `global`: el veredicto vigente es `REJECT` hasta una nueva pasada de `pull-request-review`.

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process entity_no_anemia` | 18 passed |
| `cargo test -p execute-process pull_request_review` | 10 passed |
| `cargo test -p execute-process forges` | 65 passed, 1 ignored |
| `cargo test -p sddia-qa` | 47 passed, 1 failed (`repo_capability_contracts_pass_qa_gate`: 3 contratos `memory.*` declarados en `capability-taxonomy.md` ausentes en disco; idéntico en `main`, fuera de AUT-125) |
| `sddia-qa verify-process-integrity` | OK |
| `sddia-qa verify-tools-index` | OK |
| `sddia-qa gate-entity-no-anemia --json` | FAIL esperado: 207 escaneadas, 322 hallazgos (deuda AUT-133, `audit-inventory.json`) |
| `sddia-qa gate-entity-no-anemia --if-touched --base origin/main --json` | OK (1 escaneada, 0 hallazgos) |
| `sddia-qa gate-evolution --json --range --require-synced-base` | `EVOL_CUMULO` en local (ref `origin/main` con mtime antiguo); con `--sync-base`: `EVOL_OK` |
| `sddia-qa gate-evolution --json --all` | `EVOL_OK` |

## approval_status

```text
rechazado
```
