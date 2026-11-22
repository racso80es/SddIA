---
feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
execution_id: "6931ec92-babc-4ef7-a4d1-a28d5148d33c"
tracker_ref: MAN-55
---

# Objetivos — man-55-prompt-invariants-oracle-ola0

## Misión

Estandarizar las invariantes de prompts, plantillas y handoff de agentes derivadas del diagnóstico de desacople de eventos (`KZ-PRM-01` .. `KZ-PRM-06`) junto con el oráculo ratchet de allowlists (`KZ-EST-01`), garantizando disciplina de oráculo en rojo en Ola 0, anti-evasión léxica, baseline hermético, mutación gobernada de genoma vía `entity-manager`, paridad build/host y separación estricta de roles Tekton/Argos.

## Criterios de aceptación

- **AC-1 (`K-ORA` / `KZ-PRM-01`):** Las plantillas y normas de planificación (`SddIA/templates/spec-template/spec.md`, `SddIA/library/norms/features-documentation-pattern.md`, `SddIA/agents/dedalo.md`) exigen que cada `AC-1..AC-N` declare un oráculo mecánico ejecutable (test o `sddia-qa`) con captura en rojo (`oracle-red.log`) en Ola 0 antes de autorizar cambios productivos en Ola 1, prohibiendo `grep` manual ad-hoc como único método de verificación.
- **AC-2 (`K-EVA` + `K-RATCH` / `KZ-PRM-02` + ratchet `KZ-EST-01`):** Las reglas de ejecución (`SddIA/norms/external-ai-constraints.md`, `SddIA/agents/tekton.md`) prohíben ofuscar literales (`concat!`), cambiar solo prefijos de ruta manteniendo `std::fs`, o ampliar allowlists sin laudo previo en `clarify.md`. `sddia-qa` incorpora el gate `gate-allowlist-ratchet` (y sus tests unitarios) que falla si `VIRTUAL_PATH_ALLOW`, `SCAN_EXCLUDE`, `INTEGRATION_SRC` o `allowlists/*.toml` crecen respecto a `origin/main` sin laudo explícito (`RATCHET_LAUDO`) en `clarify.md`.
- **AC-3 (`K-HERM` + `K-GEN` / `KZ-PRM-03`, `KZ-PRM-04`):** `features-documentation-pattern.md`, `external-ai-constraints.md` y `tekton.md` exigen en Ola 0 (`execution.md`) el baseline hermético (`git status --porcelain` vacío, ausencia física de `.events`/`orchestration`/`telemetry`/`_lab` en raíz, estado `systemctl --user list-units 'sddia-*'`, baseline de tests con `env -u SDDIA_EVENT_STORE -u SDDIA_EXECUTE_PROCESS_BIN`) y el protocolo estricto `entity-manager` exigiendo `handoff_hash_signature_new != handoff_hash_signature_old` (prohibido aceptar `idempotent` o editar genoma manualmente).
- **AC-4 (`K-HOST` + `K-ROLE` / `KZ-PRM-05`, `KZ-PRM-06`):** `external-ai-constraints.md`, `tekton.md`, `argos.md` y `.cursor/rules/task-closure-documental.mdc` exigen paridad build/host cuando el diff toca crates Rust que alimentan binarios residentes (rebuild debug + `--release`, restart `systemctl --user restart 'sddia-*@*.service'`, re-ejecución de `eda_event_token_hygiene` antes de `delivery-close-cycle`) y separación estricta Tekton/Argos (Tekton jamás escribe `global: APTO` en `validacion.md`, ni recorta ACs/difiere hijos sin `refine-item` previo ni degrada `F3_TECH_GATE` o filas `PARCIAL` a `non_blocking_findings`).
