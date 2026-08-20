---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
branch: feat/kaizen-consumer-ignition-filtro-c
persist_ref: docs/features/kaizen-consumer-ignition-filtro-c
pbi_ref: docs/todos/done/[KAIZEN] perfil ignición consumidor Filtro C.md
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
global: APTO
pbi_archived: true
checks:
  O1_FILTRO_C: APTO
  O2_R07: APTO
  O3_F07_IMAP: APTO
  O4_BUNDLE_F06: APTO
  O5_TRIPARTITA: APTO
  O6_SMOKE: APTO
  O7_F08_DUAL: APTO
  O8_F09_CONSTITUTION: APTO
  T6_PREPROD_VAULT: APTO
  UNIT_EMAIL_WATCHER: APTO
  UNIT_INSTANCE_CREATOR_SMOKE: APTO
git_changes:
  - .gitignore
  - SddIA/core/eda-coverage.json
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/engine/execute-process/src/engine/handlers/mod.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/norms/sddia-distribution-protocol.md
  - SddIA/process/index.md
  - SddIA/process/instance-creator.md
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/templates/constitution-consumer/CONSTITUTION.md
  - SddIA/templates/systemd/sddia-daemon@.service.template
  - interfaces/kalma2/app.js
  - start-sddia.md
  - start-sddia.sh
  - docs/features/kaizen-consumer-ignition-filtro-c/
  - docs/todos/done/[KAIZEN] perfil ignición consumidor Filtro C.md
---

# Validación — kaizen-consumer-ignition-filtro-c

## Veredicto

**APTO** — criterios O1–O8 cubiertos en lab. Ignición larga y `systemctl --user enable` real del host quedan como procedimiento operador (`systemd-f08-migration.md`); no bloquean merge.

## Evidencia breve

| Check | Evidencia |
|-------|-----------|
| O1 | `SDDIA_RUNTIME_PROFILE=consumer`; forge 403; skip forja |
| O2 | R-07 en `start-sddia.sh` |
| O3 | `plan_bootstrap_uids` + tests |
| O4 | Bundle lab 0 `.rs` + telegram |
| O5 | Norma v1.1.0 + `instance-creator` + ONBOARDING |
| O6 | Smoke `native-topology+local-qa` success |
| O7 | Dual `lab-instance-{a,b}` + plantilla daemon@ |
| O8 | `constitution-consumer` sin Windows/pwsh |
| T6 | Vault preprod mapeado; smoke true |

## Residual explícito

- Enable systemd user en host del operador (doc F-08).
- Tool binario `eda-local-topology-test` sigue stub; smoke nativo cubre gate.
- Ignición `start-sddia.sh` diferida en creator (no bloqueante).
