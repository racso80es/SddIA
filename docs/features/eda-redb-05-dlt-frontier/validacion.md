---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
branch: feat/eda-redb-05-dlt-frontier
branch_name: feat/eda-redb-05-dlt-frontier
persist_ref: docs/features/eda-redb-05-dlt-frontier
pbi_ref: docs/todos/done/[ARQUITECTURA] EDA Redb 05 — Frontera de auditoría DLT.md
document_id: PBI-EDA-REDB-05-DLT-FRONTIER
execution_id: "9584e62d-e512-47bd-8341-7d1df48ae998"
global: APTO
pbi_archived: true
checks:
  AC-5: APTO
  AC-6: APTO
  AC-10: APTO
---

# Validación — eda-redb-05-dlt-frontier

AC-5: `local_only_events_do_not_invoke_iota_and_external_seals_the_store` deja en `skipped-local-only` a `Raw_Execution_Finished`, `PullRequest_Presented` y el resto de clases locales. `Domain_Entity_Telemetry_Captured` termina en `success` con digest `local-` y sin `lab-sim-`. `PullRequest_Merged` en `filesystem` ancla con `lab-sim-` y no crea `events.redb`. En `redb`, `get_seal` devuelve ese `tx_hash` con `scope` `external_public_interest`. `dlt_telemetry_skip_config_missing_does_not_fail` y `dlt_telemetry_simulate_writes_proof` cubren la prueba local. `pull_request_iota_config_missing_still_fails` mantiene el fallo de `PullRequest_Merged` sin cartera.

AC-6: `pull_request_merged_accepts_agents_present_or_absent` valida la clase real con y sin `participating_agents` y `authorizer`. El documento de ancla los incluye solo cuando vienen en el payload y siempre lleva `merge_commit_hash`.

AC-10: `sddia-qa verify-domain-subscription-parity` responde OK sobre 39 eventos. `iota-immutable-publisher` aparece en `event-domain-subscriptions.json` solo en `PullRequest_Merged` y `System_Immunity_Certified`. `pull_request_merged_subscription_is_humanized_action_not_telegram_tool` sigue viendo IOTA en el dominio y en el espejo.
