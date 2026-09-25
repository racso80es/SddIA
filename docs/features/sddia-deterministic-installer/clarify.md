---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
purpose: Estabilización Filtro A del PBI v1.0.0; laudos de orquestador físico
version_clarify: "1.0.0"
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
pbi_uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
pbi_version: "1.0.0"
---

# Clarificación — sddia-deterministic-installer

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `7b22f932-c162-4104-b38d-b1c9c6068414`. Rama `feat/sddia-deterministic-installer`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.0.0 (Ceguera de Ejecución + Zero-Touch). Árbol limpio en `main` al init.

## Decisiones

| ID | Laudo |
|----|-------|
| L-SCRIPT | Fachada `./sddia-installer.sh` (raíz, paridad `sddia-run.sh`) + motor `SddIA/scripts/sddia-installer.sh`. Comandos `deploy` \| `teardown`. Cero prompts. Flags: `--root`, `--force`, `--vault`, `--skip-build`, `--dry-run`. |
| L-PATH | Precedencia `--root` > env `SDDIA_INSTALL_ROOT` > vault forja `SDDIA_INSTALL_ROOT` > last-resort PBI `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. Last-resort = Zero-Touch de este host; Core sigue inyectable (README § agnosticismo). Abort si `ROOT` = forja. |
| L-PAYLOAD | Extender `build-release-bundle.sh` con `--profile full-node`: descubre todos los crates nativos (`tools/`, `skills/`, `daemons/`, `engine/`, `interfaces/`) + `execute-process` + `kalma2-bridge`. Sin Filtro C (códices íntegros). Installer invoca este perfil por defecto. |
| L-CREATOR | `instance-creator` `skip_ignition:true` `runtime_profile:engineering` `vault_source` = `--vault` o bóveda forja. `try_ignite` ya es diferido; la ignición física es systemd del installer. |
| L-SYSTEMD | Tras creator: copiar unidades de `{ROOT}/.SddIA/systemd/` a `~/.config/systemd/user/` (si ausentes) y `systemctl --user enable --now sddia-<daemon>@$(systemd-escape -p ROOT).service` para factory + `email-watcher`. No hornear path de host en ExecStart (`%f`). |
| L-LIVE | Destino vivo = existe `{ROOT}/.SddIA` o `MANIFEST.json` o unidad `sddia-*@${ESC}` active. Abort fail-closed salvo `--force` (teardown atómico previo). |
| L-TEARDOWN | `stop`+`disable` de instancias `sddia-*@${ESC}.service`. No borrar plantillas factory `sddia-*@.service` compartidas. `daemon-reload`. SIGTERM residual cuyo cwd/bin ∈ `ROOT`. `rm -rf ROOT`. Forja y bóveda intactas. |
| L-VAULT | Hereda `.dev/.env` de forja si `--vault` omitido. Cero secretos en stdout/logs. |
| L-HOST | Deploy: `systemctl --user` presente. Si no `--skip-build`: `cargo` presente. Fallo = abort código ≠ 0. |
| L-TEST | Smoke `SddIA/scripts/qa/test-sddia-installer.sh` sobre `/tmp` + `--dry-run`/`--skip-build`. Cero enable real en CI. CA payload = enumeración `full-node` ⊇ `CONSUMER_BINS`. |
| L-GENOME | Este slice: scripts + docs. Cero `entity-manager`. Cero mutación `instance-creator.rs` / normas Core. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo post-CI verde. |

## Fuera (este ciclo)

Proceso Core `paciente0-undeploy`. Mutación DA-2 de `sddia-distribution-protocol`. Wipe de vaults `*.deploy-vault`. Deploy live a la ruta PBI (solo smoke `/tmp`). G5 / IOTA live.
