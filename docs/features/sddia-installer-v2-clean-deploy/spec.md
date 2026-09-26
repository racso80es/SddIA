---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
execution_id: "8ff78e98-6e4f-4859-9ca9-60ba718703ea"
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
pbi_uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
---

# Spec — sddia-installer-v2-clean-deploy

Referencia normativa: PBI §2 (R-VAULT-* … R-REG-*) y `sddia-installer-contract` 1.1.0.

## 1. Motor (`SddIA/scripts/sddia-installer.sh`)

- `stage_vault`: compone `root.dev.env` (desde `{FORGE}/.dev/.env` sin claves R-VAULT-2) + `instance.SddIA.dev.env` (`--vault` o starter-kit `.env.example` + `SDDIA_CLIENT_PORT`).
- `derive_wui_port`: `8765 + índice` en registro, salta forja `8765` y puertos registrados; reconcilia entradas huérfanas.
- `enable_units`: prerequisitos por daemon; log `skip … (missing: KEYS)`; sobrescribe plantilla si sha256 difiere.
- `teardown`: elimina plantillas `sddia-*@.service` solo si no quedan unidades `@*` activas/listadas.
- Flags nuevos: `--codex`, `--allow-shared-mailbox`, `--dry-run` enriquecido (`vault_*`, `wui_port`, `channel_keys_present`).
- Exit **4** buzón compartido; motor no exit 5 (fachada verify).

## 2. Fachada (`./sddia-installer.sh`)

- Tras `deploy` real: `./sddia-run.sh --process instance-health-verify`; exit **5** si `NO-APTO`.
- Emite `Instance_Deployed` / `Instance_Torn_Down` (pending + `route-domain-event`).
- `--no-verify` bypass verify.

## 3. Rust

- `instance_creator.rs`: `materialize_domain_profile` para `engineering` y `consumer`; `codex_slug` input.
- `kalma2-bridge`: bind fallido → exit **78**.
- Handler `instance_health_verify.rs` + registro `mod.rs`.

## 4. Plantillas systemd

- `Restart=on-failure`, `StartLimitIntervalSec=300`, `StartLimitBurst=5`, `RestartPreventExitStatus=2 78`, `ExecCondition=_precondition.sh`.

## 5. Genoma / Core (cadena autorizada + Core directo)

- EM: `instance-health-verify`, eventos domain, norm 1.1.0.
- Core: `cumulo.paths.json` (`instance.host_registry`), `event-domain-subscriptions.json`, `eda-coverage` vía EM sello.

## 6. QA

- Extender `test-sddia-installer.sh`: dry-run vault/port/skip keys (mock env).
- CI `sddia-installer-smoke` sin regresión códigos 1/2/3.
